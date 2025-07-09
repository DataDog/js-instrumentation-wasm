use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrivacyRewriteContent {
    HelperImport(String),
    DictionaryDeclaration(String),
    JSXStringDictionaryReference(String),
    PropertyKeyDictionaryReference(String),
    StringDictionaryReference(String),
    TaggedTemplateOpenerDictionaryReference(String),
    TaggedTemplateBeforeExpr(String),
    TaggedTemplateAfterExpr(String),
    TaggedTemplateTerminator(String),
    TemplateQuasiDictionaryReference(String),
    DeleteSourceMapComment(String),
}

impl PrivacyRewriteContent {
    /// Returns true if it's safe to discard this rewrite if it would increase code size.
    /// TODO: Ideally, we would only return true here for the rewrites that actually
    /// construct the dictionary, but it's currently not safe to drop the rewrites for
    /// tagged templates. To make it safe, we need to rework things so that the tagged
    /// template rewrites for a given tagged template expression can be evaluated for size
    /// as a unit, and either retained together or discarded together. Right now, they're
    /// independent, and if we dropped some but kept others we could end up generating
    /// broken code. It's not urgent to fix this, though, because tagged template rewrites
    /// are almost always a size win.
    pub fn should_only_replace_if_smaller(self: &Self) -> bool {
        match self {
            PrivacyRewriteContent::HelperImport(_) => false,
            PrivacyRewriteContent::DictionaryDeclaration(_) => false,
            PrivacyRewriteContent::JSXStringDictionaryReference(_) => true,
            PrivacyRewriteContent::PropertyKeyDictionaryReference(_) => true,
            PrivacyRewriteContent::StringDictionaryReference(_) => true,
            PrivacyRewriteContent::TaggedTemplateOpenerDictionaryReference(_) => false,
            PrivacyRewriteContent::TaggedTemplateBeforeExpr(_) => false,
            PrivacyRewriteContent::TaggedTemplateAfterExpr(_) => false,
            PrivacyRewriteContent::TaggedTemplateTerminator(_) => false,
            PrivacyRewriteContent::TemplateQuasiDictionaryReference(_) => true,
            PrivacyRewriteContent::DeleteSourceMapComment(_) => false,
        }
    }

    pub fn len(self: &Self) -> usize {
        match self {
            PrivacyRewriteContent::HelperImport(string) => string.len(),
            PrivacyRewriteContent::DictionaryDeclaration(string) => string.len(),
            PrivacyRewriteContent::JSXStringDictionaryReference(string) => string.len(),
            PrivacyRewriteContent::PropertyKeyDictionaryReference(string) => string.len(),
            PrivacyRewriteContent::StringDictionaryReference(string) => string.len(),
            PrivacyRewriteContent::TaggedTemplateOpenerDictionaryReference(string) => string.len(),
            PrivacyRewriteContent::TaggedTemplateBeforeExpr(string) => string.len(),
            PrivacyRewriteContent::TaggedTemplateAfterExpr(string) => string.len(),
            PrivacyRewriteContent::TaggedTemplateTerminator(string) => string.len(),
            PrivacyRewriteContent::TemplateQuasiDictionaryReference(string) => string.len(),
            PrivacyRewriteContent::DeleteSourceMapComment(string) => string.len(),
        }
    }
}

impl Display for PrivacyRewriteContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrivacyRewriteContent::HelperImport(string) => write!(f, "{}", string),
            PrivacyRewriteContent::DictionaryDeclaration(string) => write!(f, "{}", string),
            PrivacyRewriteContent::JSXStringDictionaryReference(string) => write!(f, "{}", string),
            PrivacyRewriteContent::PropertyKeyDictionaryReference(string) => {
                write!(f, "{}", string)
            }
            PrivacyRewriteContent::StringDictionaryReference(string) => write!(f, "{}", string),
            PrivacyRewriteContent::TaggedTemplateOpenerDictionaryReference(string) => {
                write!(f, "{}", string)
            }
            PrivacyRewriteContent::TaggedTemplateBeforeExpr(string) => write!(f, "{}", string),
            PrivacyRewriteContent::TaggedTemplateAfterExpr(string) => write!(f, "{}", string),
            PrivacyRewriteContent::TaggedTemplateTerminator(string) => write!(f, "{}", string),
            PrivacyRewriteContent::TemplateQuasiDictionaryReference(string) => {
                write!(f, "{}", string)
            }
            PrivacyRewriteContent::DeleteSourceMapComment(string) => write!(f, "{}", string),
        }
    }
}
