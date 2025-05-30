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
}

impl PrivacyRewriteContent {
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
        }
    }
}
