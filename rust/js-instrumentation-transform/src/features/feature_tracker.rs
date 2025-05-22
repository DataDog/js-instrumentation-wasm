use js_instrumentation_shared::module_kind::ModuleKeywordUsage;

pub struct FeatureTracker {
    pub module_keyword_usage: ModuleKeywordUsage,
}

impl FeatureTracker {
    pub fn new() -> FeatureTracker {
        FeatureTracker {
            module_keyword_usage: ModuleKeywordUsage {
                es_modules: false,
                require: false,
            },
        }
    }

    pub fn observed_require(self: &mut Self) {
        self.module_keyword_usage.require = true;
    }

    pub fn observed_export_or_import(self: &mut Self) {
        self.module_keyword_usage.es_modules = true;
    }
}
