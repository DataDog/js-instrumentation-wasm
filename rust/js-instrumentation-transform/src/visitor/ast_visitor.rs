use swc_common::{Span, Spanned};
use swc_ecma_ast::{
    CallExpr, Callee, ExportAll, Expr, Ident, ImportDecl, JSXAttrValue, JSXElementChild, JSXText,
    Lit, NamedExport, Program, PropName, Stmt, Str, TaggedTpl, Tpl, TsEnumDecl, TsInterfaceDecl,
    TsModuleName, TsType,
};
use swc_ecma_visit::{Visit, VisitWith};

use crate::{
    dictionary::DictionaryTracker,
    features::FeatureTracker,
    identifiers::IdentifierTracker,
    rewrite::{Rewrite, RewriteTracker},
};

pub fn visit(
    program: Program,
    dictionary_tracker: &mut DictionaryTracker,
    feature_tracker: &mut FeatureTracker,
    identifier_tracker: &mut IdentifierTracker,
    rewrite_tracker: &mut RewriteTracker,
) {
    let mut visitor = ASTVisitor {
        dictionary_tracker,
        feature_tracker,
        identifier_tracker,
        rewrite_tracker,
    };
    program.visit_with(&mut visitor);
}

struct ASTVisitor<'a> {
    dictionary_tracker: &'a mut DictionaryTracker,
    feature_tracker: &'a mut FeatureTracker,
    identifier_tracker: &'a mut IdentifierTracker,
    rewrite_tracker: &'a mut RewriteTracker,
}

impl<'a> ASTVisitor<'a> {
    /// Perform an action in a context in which we won't add new strings to the dictionary.
    fn in_uncollected_scope<F: FnOnce(&mut Self)>(self: &mut Self, action: F) {
        self.dictionary_tracker.enter_uncollected_scope();
        action(self);
        self.dictionary_tracker.exit_uncollected_scope();
    }

    /// Perform an action in a context in which we won't rewrite code.
    fn in_unrewritten_scope<F: FnOnce(&mut Self)>(self: &mut Self, action: F) {
        self.rewrite_tracker.enter_unrewritten_scope();
        action(self);
        self.rewrite_tracker.exit_unrewritten_scope();
    }
}

impl<'a> Visit for ASTVisitor<'a> {
    fn visit_ident(&mut self, node: &Ident) {
        self.identifier_tracker.add_ident(node);
    }

    fn visit_str(&mut self, node: &Str) {
        match node {
            Str { raw, span, value } => {
                if let Some(index) = self.dictionary_tracker.maybe_add_string(&raw, &value) {
                    self.rewrite_tracker.emit(Rewrite::string(index, *span));
                }
            }
        }
    }

    fn visit_prop_name(&mut self, node: &PropName) {
        match node {
            PropName::Str(Str { raw, span, value }) => {
                if let Some(index) = self.dictionary_tracker.maybe_add_string(&raw, &value) {
                    self.rewrite_tracker
                        .emit(Rewrite::property_key(index, *span));
                }
            }
            _ => {
                node.visit_children_with(self);
            }
        }
    }

    fn visit_tpl(&mut self, node: &Tpl) {
        let mut quasi_iter = node.quasis.iter();
        let mut expr_iter = node.exprs.iter();
        loop {
            let next_quasi = quasi_iter.next();
            if let Some(quasi) = next_quasi {
                if let Some(index) = self.dictionary_tracker.maybe_add_template_quasi(&quasi.raw) {
                    self.rewrite_tracker
                        .emit(Rewrite::template_quasi(index, quasi.span));
                }
            }

            let next_expr = expr_iter.next();
            if let Some(expr) = next_expr {
                expr.visit_children_with(self);
            }

            if let (None, None) = (next_quasi, next_expr) {
                break;
            }
        }
    }

    fn visit_tagged_tpl(&mut self, node: &TaggedTpl) {
        let quasis = node
            .tpl
            .quasis
            .iter()
            .map(|quasi| quasi.raw.clone())
            .collect();
        let index = self.dictionary_tracker.maybe_add_tagged_template(&quasis);
        if let None = index {
            node.visit_children_with(self);
            return;
        }

        node.tag.visit_with(self);

        self.rewrite_tracker.emit(Rewrite::tagged_template_opener(
            index.unwrap(),
            Span {
                lo: node.tpl.span.lo,
                hi: node.tpl.span.lo,
            },
        ));

        for expr in &node.tpl.exprs {
            let expr_span = expr.span();

            self.rewrite_tracker
                .emit(Rewrite::tagged_template_before_expr(Span {
                    lo: expr_span.lo,
                    hi: expr_span.lo,
                }));

            expr.visit_children_with(self);

            self.rewrite_tracker
                .emit(Rewrite::tagged_template_after_expr(Span {
                    lo: expr_span.hi,
                    hi: expr_span.hi,
                }));
        }

        self.rewrite_tracker
            .emit(Rewrite::tagged_template_terminator(Span {
                lo: node.tpl.span.hi,
                hi: node.tpl.span.hi,
            }));
    }

    fn visit_jsx_attr_value(&mut self, node: &JSXAttrValue) {
        match &node {
            JSXAttrValue::Lit(Lit::Str(Str { raw, span, value })) => {
                if let Some(index) = self.dictionary_tracker.maybe_add_jsx_attribute(raw, value) {
                    self.rewrite_tracker.emit(Rewrite::jsx_string(index, *span));

                    // Don't recurse; we don't want to treat this as an ordinary string.
                    return;
                }
            }
            _ => {}
        }

        node.visit_children_with(self);
    }

    fn visit_jsx_element_child(&mut self, node: &JSXElementChild) {
        match &node {
            JSXElementChild::JSXText(JSXText { raw, span, value }) => {
                if let Some(index) = self.dictionary_tracker.maybe_add_jsx_text(raw, value) {
                    self.rewrite_tracker.emit(Rewrite::jsx_string(index, *span));
                }
            }
            _ => {}
        }

        node.visit_children_with(self);
    }

    fn visit_ts_enum_decl(&mut self, node: &TsEnumDecl) {
        // It's OK to collect strings in enum declarations, but we should not
        // ever rewrite them, because they're subject to some weird constraints.
        self.in_unrewritten_scope(|this| node.visit_children_with(this));
    }

    fn visit_import_decl(&mut self, node: &ImportDecl) {
        // Don't collect string literals inside import declarations.
        self.feature_tracker.observed_export_or_import();
        self.in_uncollected_scope(|this| node.visit_children_with(this));
    }

    fn visit_named_export(&mut self, node: &NamedExport) {
        // Don't collect string literals inside named exports.
        self.feature_tracker.observed_export_or_import();
        self.in_uncollected_scope(|this| node.visit_children_with(this));
    }

    fn visit_export_all(&mut self, node: &ExportAll) {
        // Don't collect string literals inside export all declarations.
        self.feature_tracker.observed_export_or_import();
        self.in_uncollected_scope(|this| node.visit_children_with(this));
    }

    fn visit_ts_type(&mut self, _node: &TsType) {
        // Don't collect string literals inside TypeScript types.
        // Note that we don't need to recurse with while_filtered() here,
        // because there's nothing we need to collect inside TypeScript types.
        return;
    }

    fn visit_ts_interface_decl(&mut self, _node: &TsInterfaceDecl) {
        // Don't collect string literals inside TypeScript interface declarations.
        // Note that we don't need to recurse with while_filtered() here,
        // because there's nothing we need to collect inside TypeScript types.
        return;
    }

    fn visit_ts_module_name(&mut self, _node: &TsModuleName) {
        // Don't treat TypeScript module names as string literals, even though
        // string literals can be used in this position syntactically.
        return;
    }

    fn visit_stmt(&mut self, node: &Stmt) {
        match node {
            Stmt::Expr(stmt) => {
                match *stmt.expr {
                    Expr::Lit(Lit::Str(_)) => {
                        // Don't collect string literals in a statement that consists only
                        // of a string literal. This is a directive like "use strict".
                        // Note that we don't need to recurse with while_filtered() here,
                        // because there's nothing inside a string literal.
                        return;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        node.visit_children_with(self);
    }

    fn visit_call_expr(&mut self, node: &CallExpr) {
        match &node.callee {
            Callee::Expr(expr) => {
                match **expr {
                    Expr::Ident(ref ident) => {
                        if ident.sym.as_str() == "eval" {
                            // Don't collect strings inside eval() calls.
                            self.in_uncollected_scope(|this| node.visit_children_with(this));
                            return;
                        }
                        if ident.sym.as_str() == "require" {
                            // Don't collect strings inside require() calls.
                            self.feature_tracker.observed_require();
                            self.in_uncollected_scope(|this| node.visit_children_with(this));
                            return;
                        }
                    }
                    _ => {}
                }
            }
            Callee::Import(_) => {
                // Don't collect strings inside import() expressions.
                self.feature_tracker.observed_export_or_import();
                self.in_uncollected_scope(|this| node.visit_children_with(this));
                return;
            }
            _ => {}
        }

        node.visit_children_with(self);
    }
}

#[cfg(test)]
mod tests {
    use swc_common::source_map::SmallPos;
    use swc_common::BytePos;

    use crate::{
        dictionary::DictionaryEntry,
        input::{build_parser, InputFile},
    };

    use super::*;

    fn walk_code(code: &str) -> (DictionaryTracker, RewriteTracker) {
        let input_file = InputFile::new("test.jsx", code);
        let mut parser = build_parser(&input_file, &Default::default());
        let program = parser.parse_program().unwrap();

        let mut dictionary_tracker = DictionaryTracker::new();
        let mut feature_tracker = FeatureTracker::new();
        let mut identifier_tracker = IdentifierTracker::new(vec![]);
        let mut rewrite_tracker = RewriteTracker::new();

        visit(
            program,
            &mut dictionary_tracker,
            &mut feature_tracker,
            &mut identifier_tracker,
            &mut rewrite_tracker,
        );

        (dictionary_tracker, rewrite_tracker)
    }

    #[test]
    fn handles_property_key_inside_function_inside_jsx_attribute() {
        let (dictionary_tracker, rewrite_tracker) = walk_code(
            r#"
              export function MyComponent(props) {
                return (
                  <ErrorHandler
                    onError={(info) => {
                      reportError({ 'info': info });
                    }}
                />);
              }
            "#,
        );
        assert_eq!(
            dictionary_tracker
                .strings
                .keys()
                .cloned()
                .collect::<Vec<DictionaryEntry>>(),
            vec![DictionaryEntry::String("'info'".into())]
        );
        assert_eq!(
            rewrite_tracker.rewrites,
            vec![Rewrite::property_key(
                0,
                Span::new(BytePos::from_u32(187), BytePos::from_u32(193))
            )]
        );
    }
}
