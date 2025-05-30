use js_instrumentation_rewrite::rewrite::Rewrite;
use js_instrumentation_shared::log::debug_log;

use super::PrivacyRewriteTemplate;

pub struct RewriteTracker {
    rewrites: Vec<Rewrite<PrivacyRewriteTemplate>>,
    in_unrewritten_scopes: usize,
}

impl RewriteTracker {
    pub fn new(rewrites: Vec<Rewrite<PrivacyRewriteTemplate>>) -> RewriteTracker {
        RewriteTracker {
            rewrites,
            in_unrewritten_scopes: 0,
        }
    }

    pub fn enter_unrewritten_scope(self: &mut Self) {
        self.in_unrewritten_scopes += 1;
    }

    pub fn exit_unrewritten_scope(self: &mut Self) {
        if self.in_unrewritten_scopes == 0 {
            debug_log("exit_unrewritten_scope called outside any uncollected scope.");
            return;
        }
        self.in_unrewritten_scopes -= 1;
    }

    pub fn emit(self: &mut Self, rewrite: Rewrite<PrivacyRewriteTemplate>) {
        if self.in_unrewritten_scopes > 0 {
            return;
        }

        self.rewrites.push(rewrite);
    }

    pub fn take(self: Self) -> Vec<Rewrite<PrivacyRewriteTemplate>> {
        self.rewrites
    }
}
