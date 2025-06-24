use serde::{Deserialize, Serialize};

use crate::module_kind::ModuleKind;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputOptions {
    /// Some: use the given module kind when it can't be determined from the filename.
    /// None: default to ESM.
    pub module: Option<ModuleKind>,

    /// True: enable JSX. False: disable JSX. None: guess based on filename.
    pub jsx: Option<bool>,

    /// True: enable TypeScript. False: disable TypeScript. None: guess based on filename.
    pub typescript: Option<bool>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
pub enum HelperFunctionSource {
    Expression {
        code: String,
    },
    Import {
        cjs_module: String,
        esm_module: String,
        func: String,
    },
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyOptions {
    pub add_to_dictionary_helper: HelperFunctionSource,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformOptions {
    pub input: InputOptions,
    pub privacy: PrivacyOptions,
}

impl Default for TransformOptions {
    fn default() -> Self {
        TransformOptions {
            input: InputOptions {
                module: None,
                jsx: Some(true),
                typescript: Some(true),
            },
            privacy: PrivacyOptions {
                add_to_dictionary_helper: HelperFunctionSource::Import {
                    cjs_module: "datadog:privacy-helpers.cjs".into(),
                    esm_module: "datadog:privacy-helpers.mjs".into(),
                    func: "$".into(),
                },
            },
        }
    }
}
