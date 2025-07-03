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
pub struct OutputOptions {
    /// True: inline the source map in the transformed file.
    pub inline_source_map: bool,

    /// True: embed the source code in the source map.
    pub embed_code_in_source_map: bool,
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
pub struct InstrumentationOptions {
    pub input: InputOptions,
    pub output: OutputOptions,
    pub privacy: PrivacyOptions,
}

impl Default for InstrumentationOptions {
    fn default() -> Self {
        InstrumentationOptions {
            input: InputOptions {
                module: None,
                jsx: Some(true),
                typescript: Some(true),
            },
            output: OutputOptions {
                inline_source_map: false,
                embed_code_in_source_map: true,
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
