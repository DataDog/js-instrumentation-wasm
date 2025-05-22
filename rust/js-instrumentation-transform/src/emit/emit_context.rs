use js_instrumentation_shared::module_kind::ModuleKind;

use crate::dictionary::OptimizedDictionary;

pub struct EmitContext {
    pub dictionary: OptimizedDictionary,
    pub dictionary_identifier: String,
    pub helpers_module: String,
    pub helper_identifier: String,
    pub module_kind: ModuleKind,
}

impl EmitContext {
    pub fn new(
        dictionary: OptimizedDictionary,
        dictionary_identifier: String,
        helpers_module: &String,
        helper_identifier: String,
        module_kind: ModuleKind,
    ) -> EmitContext {
        EmitContext {
            dictionary,
            dictionary_identifier,
            helpers_module: helpers_module.into(),
            helper_identifier,
            module_kind,
        }
    }
}
