use std::fmt::Write;

use js_instrumentation_shared::{debug_log, InputFile};
use swc_common::BytePos;

use crate::{rewrite::Rewrite, rewrite_content::RewriteContent};

pub struct RewritePlan<Content: RewriteContent> {
    rewrites: Vec<Rewrite<Content>>,
}

pub fn build_rewrite_plan<Content, RewriteIterable>(
    rewrite_iterable: RewriteIterable,
) -> RewritePlan<Content>
where
    Content: RewriteContent,
    RewriteIterable: IntoIterator<Item = Rewrite<Content>>,
{
    // Sort the rewrites.
    let mut rewrites: Vec<Rewrite<Content>> = rewrite_iterable.into_iter().collect();
    rewrites.sort_unstable();

    // Filter out rewrites that overlap with previous rewrites, since they would otherwise
    // conflict.
    let mut prev_hi: Option<BytePos> = None;
    let rewrites: Vec<Rewrite<Content>> = rewrites
        .into_iter()
        .filter(|rewrite| {
            let should_keep = match prev_hi {
                None => true,
                Some(prev_hi) if prev_hi > *rewrite.lo() => false,
                Some(_) => true,
            };
            if should_keep {
                prev_hi = Some(*rewrite.hi());
            } else {
                debug_log(&format!("Skipping rewrite due to overlap: {}", rewrite));
            }
            should_keep
        })
        .collect();

    // The result should be a safe rewrite plan.
    RewritePlan { rewrites }
}

impl<Content: RewriteContent> RewritePlan<Content> {
    pub fn apply<'a>(self: &Self, input_file: &mut InputFile<'a>) -> String {
        let mut input_pos = input_file.start_pos;
        let mut output = String::new();

        for rewrite in &self.rewrites {
            output += input_file.slice(input_pos, *rewrite.lo());
            let _ = write!(&mut output, "{}", rewrite.content());
            input_pos = *rewrite.hi();
        }

        output += input_file.slice(input_pos, input_file.end_pos);

        return output;
    }
}
