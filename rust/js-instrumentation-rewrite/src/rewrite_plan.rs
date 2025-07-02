use js_instrumentation_shared::{debug_log, InputFile};
use swc_common::BytePos;
use swc_core::base::sourcemap::SourceMap;

use crate::{rewrite::Rewrite, rewrite_content::RewriteContent, rewrite_output::RewriteOutput};

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
    pub fn apply<'a>(
        self: &Self,
        input_file: &mut InputFile<'a>,
        token_positions: Vec<BytePos>,
        embed_code_in_source_map: bool,
    ) -> (String, SourceMap) {
        let embedded_code = if embed_code_in_source_map {
            Some(vec![
                Some(String::from(input_file.code).into()),
                Some("".into()),
            ])
        } else {
            None
        };

        let mut output = RewriteOutput::new(input_file, token_positions);

        for rewrite in &self.rewrites {
            match rewrite {
                Rewrite::Replace { content, span } => {
                    output.emit_input_until(span.lo());
                    output.emit_replacement_until(span.hi(), &format!("{}", content));
                }
                Rewrite::Insert { content, pos } => {
                    output.emit_input_until(*pos);
                    output.emit_insertion(&format!("{}", content));
                }
            }
        }

        let (rewritten_code, source_map_tokens) = output.finish();

        let mut source_map = SourceMap::new(
            None,
            source_map_tokens,
            vec![],
            vec![String::from(input_file.name).into(), "".into()],
            embedded_code,
        );
        source_map.add_to_ignore_list(1);

        return (rewritten_code, source_map);
    }
}
