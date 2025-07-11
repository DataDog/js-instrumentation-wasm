use serde::{Deserialize, Serialize};

use crate::{
    filetype::{filename_is_explicitly_cjs, filename_is_explicitly_esm},
    instrumentation_options::InstrumentationOptions,
};

#[derive(Clone, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ModuleKind {
    CJS,
    ESM,
}

pub struct ModuleKeywordUsage {
    pub cjs: bool,
    pub esm: bool,
}

pub fn module_kind_for(
    filename: &str,
    options: &InstrumentationOptions,
    keyword_usage: Option<ModuleKeywordUsage>,
) -> ModuleKind {
    match (&options.input.module, keyword_usage) {
        // If the user specified a module kind, use it.
        (Some(ref kind), _) => kind.clone(),

        // Infer the module kind from the filename if possible.
        _ if filename_is_explicitly_cjs(filename) => ModuleKind::CJS,
        _ if filename_is_explicitly_esm(filename) => ModuleKind::ESM,

        // If the file contained `import` or `export`, treat it as ESM.
        (None, Some(ModuleKeywordUsage { esm: true, .. })) => ModuleKind::ESM,

        // If it contained `require()` or `exports`, but no `import` or `export`, treat it as CJS.
        (
            None,
            Some(ModuleKeywordUsage {
                esm: false,
                cjs: true,
            }),
        ) => ModuleKind::CJS,

        // Otherwise, default to ESM. It seems that today, overall, this is the slightly
        // more compatible default.
        (None, _) => ModuleKind::ESM,
    }
}
