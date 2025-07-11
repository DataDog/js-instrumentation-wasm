use js_instrumentation_shared::module_kind::ModuleKeywordUsage;

pub struct FeatureTracker {
    pub module_keyword_usage: ModuleKeywordUsage,
}

impl FeatureTracker {
    pub fn new() -> FeatureTracker {
        FeatureTracker {
            module_keyword_usage: ModuleKeywordUsage {
                cjs: false,
                esm: false,
            },
        }
    }

    pub fn observed_cjs_exports_or_require(self: &mut Self) {
        self.module_keyword_usage.cjs = true;
    }

    pub fn observed_esm_export_or_import(self: &mut Self) {
        self.module_keyword_usage.esm = true;
    }
}
