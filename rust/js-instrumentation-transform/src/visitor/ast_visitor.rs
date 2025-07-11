use js_instrumentation_shared::InputFile;
use swc_atoms::Atom;
use swc_common::{BytePos, Span, Spanned};
use swc_ecma_ast::{
    CallExpr, Callee, ExportAll, Expr, Ident, IdentName, ImportDecl, JSXAttr, JSXAttrName,
    JSXAttrValue, JSXElement, JSXElementChild, JSXElementName, JSXText, Lit, NamedExport, Program,
    PropName, Stmt, Str, TaggedTpl, Tpl, TsEnumDecl, TsInterfaceDecl, TsModuleName, TsType,
};
use swc_ecma_visit::{Visit, VisitWith};

use crate::{
    dictionary::DictionaryTracker,
    features::FeatureTracker,
    identifiers::IdentifierTracker,
    rewrite::{
        replace_jsx_string_with_dictionary_ref, replace_property_key_with_dictionary_ref,
        replace_string_with_dictionary_ref, replace_tagged_template_after_expr_marker,
        replace_tagged_template_before_expr_marker,
        replace_tagged_template_opener_with_dictionary_ref, replace_tagged_template_terminator,
        replace_template_quasi_with_dictionary_ref, RewriteTracker,
    },
};

pub fn visit<'a, 'b>(
    program: &Program,
    input_file: &'a mut InputFile<'b>,
    dictionary_tracker: &'a mut DictionaryTracker,
    feature_tracker: &'a mut FeatureTracker,
    identifier_tracker: &'a mut IdentifierTracker,
    rewrite_tracker: &'a mut RewriteTracker,
) {
    let mut visitor = ASTVisitor {
        input_file,
        dictionary_tracker,
        feature_tracker,
        identifier_tracker,
        rewrite_tracker,
    };
    program.visit_with(&mut visitor);
}

struct ASTVisitor<'a, 'b> {
    input_file: &'a mut InputFile<'b>,
    dictionary_tracker: &'a mut DictionaryTracker,
    feature_tracker: &'a mut FeatureTracker,
    identifier_tracker: &'a mut IdentifierTracker,
    rewrite_tracker: &'a mut RewriteTracker,
}

impl<'a, 'b> ASTVisitor<'a, 'b> {
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

impl<'a, 'b> Visit for ASTVisitor<'a, 'b> {
    fn visit_span(&mut self, span: &Span) {
        // Track the starting position of all AST nodes; this roughly
        // gives us the starting position of all tokens, which we need
        // to generate high-quality source maps. (The TypeScript
        // compiler does this as well.)
        self.rewrite_tracker.add_token_position(span.lo);
    }

    fn visit_ident(&mut self, node: &Ident) {
        // Track all identifiers, so that we can generate unique
        // identifiers later.
        self.identifier_tracker.add_ident(node);

        // If we see an identifier named "exports", assume it's the CommonJS "exports". (It may not
        // be, if this is really an ES module, but in that case we'll generally observe an ESM
        // "import" or "export" and correctly treat the module as ESM anyway.)
        if node.sym.as_str() == "exports" {
            self.feature_tracker.observed_cjs_exports_or_require();
        }

        // Track both the starting and ending position of all identifiers, so that we generate
        // source maps that can handle identifier renaming well. (The TypeScript compiler does this
        // as well.)
        self.rewrite_tracker.add_token_position(node.span.lo);
        self.rewrite_tracker.add_token_position(node.span.hi);
    }

    fn visit_ident_name(&mut self, node: &IdentName) {
        // Track all identifiers, so that we can generate unique
        // identifiers later.
        self.identifier_tracker.add_ident_name(node);

        // If we see an identifier named "exports", assume it's the CommonJS "exports". (It may not
        // be, if this is really an ES module, but in that case we'll generally observe an ESM
        // "import" or "export" and correctly treat the module as ESM anyway.)
        if node.sym.as_str() == "exports" {
            self.feature_tracker.observed_cjs_exports_or_require();
        }

        // Track both the starting and ending position of all identifiers, so that we generate
        // source maps that can handle identifier renaming well. (The TypeScript compiler does this
        // as well.)
        self.rewrite_tracker.add_token_position(node.span.lo);
        self.rewrite_tracker.add_token_position(node.span.hi);
    }

    fn visit_str(&mut self, node: &Str) {
        match node {
            Str { raw, span, value } => {
                if let Some(index) = self
                    .dictionary_tracker
                    .maybe_add_string(&raw, &value, &span)
                {
                    let may_follow_keyword = self.input_file.may_follow_keyword(span.lo);
                    self.rewrite_tracker
                        .emit(replace_string_with_dictionary_ref(
                            index,
                            *span,
                            may_follow_keyword,
                        ));
                }

                // Track both the start and ending position since we rewrite this kind of token.
                self.rewrite_tracker.add_token_position(span.lo);
                self.rewrite_tracker.add_token_position(span.hi);
            }
        }
    }

    fn visit_prop_name(&mut self, node: &PropName) {
        match node {
            PropName::Str(Str { raw, span, value }) => {
                if let Some(index) = self
                    .dictionary_tracker
                    .maybe_add_string(&raw, &value, &span)
                {
                    self.rewrite_tracker
                        .emit(replace_property_key_with_dictionary_ref(index, *span));
                }

                // Track both the start and ending position since we rewrite this kind of token.
                self.rewrite_tracker.add_token_position(span.lo);
                self.rewrite_tracker.add_token_position(span.hi);
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
                if let Some(index) = self
                    .dictionary_tracker
                    .maybe_add_template_quasi(&quasi.raw, &quasi.span)
                {
                    self.rewrite_tracker
                        .emit(replace_template_quasi_with_dictionary_ref(
                            index, quasi.span,
                        ));
                }

                // Track both the start and ending position since we rewrite this kind of token.
                self.rewrite_tracker.add_token_position(quasi.span.lo);
                self.rewrite_tracker.add_token_position(quasi.span.hi);
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
            .map(|quasi| {
                // Track both the start and ending position since we rewrite this kind of token.
                self.rewrite_tracker.add_token_position(quasi.span.lo);
                self.rewrite_tracker.add_token_position(quasi.span.hi);

                quasi.raw.clone()
            })
            .collect();
        let index = self
            .dictionary_tracker
            .maybe_add_tagged_template(&quasis, &node.span);
        if let None = index {
            node.visit_children_with(self);
            return;
        }

        node.tag.visit_with(self);

        let opening_backtick_lo = node.tpl.span.lo;
        let opening_backtick_hi = self.input_file.next_char_pos(opening_backtick_lo);

        self.rewrite_tracker
            .emit(replace_tagged_template_opener_with_dictionary_ref(
                index.unwrap(),
                Span {
                    lo: opening_backtick_lo,
                    hi: opening_backtick_hi,
                },
            ));

        let mut prev_hi: BytePos = opening_backtick_hi;

        for expr in &node.tpl.exprs {
            let expr_span = expr.span();

            self.rewrite_tracker
                .emit(replace_tagged_template_before_expr_marker(Span {
                    lo: prev_hi,
                    hi: expr_span.lo,
                }));

            expr.visit_children_with(self);

            let rbrace_lo = expr_span.hi;
            let rbrace_hi = self.input_file.next_char_pos(rbrace_lo);

            self.rewrite_tracker
                .emit(replace_tagged_template_after_expr_marker(Span {
                    lo: rbrace_lo,
                    hi: rbrace_hi,
                }));

            prev_hi = rbrace_hi;
        }

        self.rewrite_tracker
            .emit(replace_tagged_template_terminator(Span {
                lo: prev_hi,
                hi: node.tpl.span.hi,
            }));
    }

    fn visit_jsx_attr(&mut self, node: &JSXAttr) {
        match &node.name {
            JSXAttrName::Ident(ident) if is_uncollected_jsx_attr(&ident.sym) => {
                self.in_uncollected_scope(|this| node.visit_children_with(this));
            }
            JSXAttrName::JSXNamespacedName(name) if is_uncollected_jsx_attr(&name.name.sym) => {
                self.in_uncollected_scope(|this| node.visit_children_with(this));
            }
            _ => {
                node.visit_children_with(self);
            }
        }
    }

    fn visit_jsx_attr_value(&mut self, node: &JSXAttrValue) {
        match &node {
            JSXAttrValue::Lit(Lit::Str(Str { raw, span, value })) => {
                if let Some(index) = self
                    .dictionary_tracker
                    .maybe_add_jsx_attribute(raw, value, span)
                {
                    self.rewrite_tracker
                        .emit(replace_jsx_string_with_dictionary_ref(index, *span));

                    // Track both the start and ending position since we rewrite this kind of token.
                    self.rewrite_tracker.add_token_position(span.lo);
                    self.rewrite_tracker.add_token_position(span.hi);

                    // Don't recurse; we don't want to treat this as an ordinary string.
                    return;
                }
            }
            _ => {}
        }

        node.visit_children_with(self);
    }

    fn visit_jsx_element(&mut self, node: &JSXElement) {
        match &node.opening.name {
            JSXElementName::Ident(ident) if is_uncollected_jsx_element(&ident.sym) => {
                self.in_uncollected_scope(|this| node.visit_children_with(this));
            }
            JSXElementName::JSXNamespacedName(name)
                if is_uncollected_jsx_element(&name.name.sym) =>
            {
                self.in_uncollected_scope(|this| node.visit_children_with(this));
            }
            _ => {
                node.visit_children_with(self);
            }
        }
    }

    fn visit_jsx_element_child(&mut self, node: &JSXElementChild) {
        match &node {
            JSXElementChild::JSXText(JSXText { raw, span, value }) => {
                if let Some(index) = self.dictionary_tracker.maybe_add_jsx_text(raw, value, span) {
                    self.rewrite_tracker
                        .emit(replace_jsx_string_with_dictionary_ref(index, *span));
                }

                // Track both the start and ending position since we rewrite this kind of token.
                self.rewrite_tracker.add_token_position(span.lo);
                self.rewrite_tracker.add_token_position(span.hi);
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
        self.feature_tracker.observed_esm_export_or_import();
        self.in_uncollected_scope(|this| node.visit_children_with(this));
    }

    fn visit_named_export(&mut self, node: &NamedExport) {
        // Don't collect string literals inside named exports.
        self.feature_tracker.observed_esm_export_or_import();
        self.in_uncollected_scope(|this| node.visit_children_with(this));
    }

    fn visit_export_all(&mut self, node: &ExportAll) {
        // Don't collect string literals inside export all declarations.
        self.feature_tracker.observed_esm_export_or_import();
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
                    Expr::Lit(Lit::Str(Str { span, .. })) => {
                        // Track both the start and ending position since we rewrite this kind of token.
                        self.rewrite_tracker.add_token_position(span.lo);
                        self.rewrite_tracker.add_token_position(span.hi);

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
                        // Don't collect strings inside certain function calls.
                        match ident.sym.as_str() {
                            "eval" => {
                                self.in_uncollected_scope(|this| node.visit_children_with(this));
                                return;
                            }
                            "require" => {
                                self.feature_tracker.observed_cjs_exports_or_require();
                                self.in_uncollected_scope(|this| node.visit_children_with(this));
                                return;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Callee::Import(_) => {
                // Don't collect strings inside import() expressions.
                self.feature_tracker.observed_esm_export_or_import();
                self.in_uncollected_scope(|this| node.visit_children_with(this));
                return;
            }
            _ => {}
        }

        node.visit_children_with(self);
    }

    fn visit_new_expr(&mut self, node: &swc_ecma_ast::NewExpr) {
        match *node.callee {
            Expr::Ident(ref ident) => {
                // Don't collect strings inside certain constructor calls.
                match ident.sym.as_str() {
                    "Function" | "RegExp" => {
                        self.in_uncollected_scope(|this| node.visit_children_with(this));
                        return;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        node.visit_children_with(self);
    }
}

fn is_uncollected_jsx_element(atom: &Atom) -> bool {
    match atom.as_str() {
        "g" | "path" => true,
        _ => false,
    }
}

fn is_uncollected_jsx_attr(atom: &Atom) -> bool {
    match atom.as_str() {
        "class" | "className" | "d" | "id" | "src" | "srcset" | "style" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use js_instrumentation_shared::{build_parser, InputFile};
    use swc_common::comments::SingleThreadedComments;
    use swc_common::source_map::SmallPos;
    use swc_common::BytePos;

    use crate::comments::process_comments;
    use crate::dictionary::DictionaryEntry;

    use super::*;

    fn walk_code(code: &str) -> (DictionaryTracker, RewriteTracker) {
        let mut input_file = InputFile::new("test.jsx", code);
        let comments: SingleThreadedComments = Default::default();
        let mut parser = build_parser(&input_file, &comments, &Default::default());
        let program = parser.parse_program().unwrap();
        let (directive_set, _) = process_comments(&input_file, &comments);

        let mut dictionary_tracker = DictionaryTracker::new(directive_set);
        let mut feature_tracker = FeatureTracker::new();
        let mut identifier_tracker = IdentifierTracker::new(vec![]);
        let mut rewrite_tracker = RewriteTracker::new(vec![]);

        visit(
            &program,
            &mut input_file,
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
            rewrite_tracker.take().0,
            vec![replace_property_key_with_dictionary_ref(
                0,
                Span::new(BytePos::from_u32(187), BytePos::from_u32(193))
            )]
        );
    }
}
