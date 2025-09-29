use js_instrumentation_rewrite::rewrite::Rewrite;
use js_instrumentation_shared::log::debug_log;
use swc_common::BytePos;

use super::PrivacyRewriteTemplate;

pub struct RewriteTracker {
    in_unrewritten_scopes: usize,
    rewrites: Vec<Rewrite<PrivacyRewriteTemplate>>,
    token_positions: Vec<BytePos>,
}

impl RewriteTracker {
    pub fn new() -> RewriteTracker {
        RewriteTracker {
            in_unrewritten_scopes: 0,
            rewrites: Vec::new(),
            token_positions: Vec::new(),
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

    pub fn add_token_position(self: &mut Self, pos: BytePos) {
        match self.token_positions.last() {
            Some(last_pos) if *last_pos == pos => {
                // Don't add the same token position more than once.
                // (We'll do additional deduplication later, but to
                // avoid wasting memory it's nice to filter out
                // duplicates up front with a cheap check.)
            }
            _ => {
                self.token_positions.push(pos);
            }
        }
    }

    pub fn take(mut self: Self) -> (Vec<Rewrite<PrivacyRewriteTemplate>>, Vec<BytePos>) {
        self.token_positions.sort_unstable();
        self.token_positions.dedup();
        (self.rewrites, self.token_positions)
    }
}
