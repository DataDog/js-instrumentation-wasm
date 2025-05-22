use serde::{Deserialize, Serialize};

use crate::module_kind::ModuleKind;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransformStrategy {
    AST,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformOptions {
    /// Some: use the given module kind when it can't be determined from the filename.
    /// None: default to ESM.
    pub module: Option<ModuleKind>,

    /// True: enable JSX. False: disable JSX. None: guess based on filename.
    pub jsx: Option<bool>,

    /// True: enable TypeScript. False: disable TypeScript. None: guess based on filename.
    pub typescript: Option<bool>,

    /// The name of the "add to dictionary" helper to use.
    pub add_to_dictionary_helper: String,

    /// The name of the helpers module to auto-inject.
    pub helpers_module: String,

    /// Which source code transformation strategy to use.
    /// None: default to AST.
    pub transform_strategy: Option<TransformStrategy>,
}

impl Default for TransformOptions {
    fn default() -> Self {
        TransformOptions {
            module: None,
            jsx: Some(true),
            typescript: Some(true),

            add_to_dictionary_helper: "$".into(),
            helpers_module: "datadog:privacy-helpers".into(),
            transform_strategy: Some(TransformStrategy::AST),
        }
    }
}
