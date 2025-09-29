use js_instrumentation_shared::{debug_log, InputFile};
use swc_common::BytePos;
use swc_core::base::sourcemap::SourceMap;

use crate::{rewrite::Rewrite, rewrite_content::RewriteContent, rewrite_output::RewriteOutput};

pub struct RewritePlan<Content: RewriteContent> {
    rewrites: Vec<Rewrite<Content>>,
    source_positions_used_in_mappings: Vec<BytePos>,
}

pub fn build_rewrite_plan<Content, HeaderRewriteIterable, BodyRewriteIterable>(
    header_iterable: HeaderRewriteIterable,
    body_iterable: BodyRewriteIterable,
) -> RewritePlan<Content>
where
    Content: RewriteContent,
    HeaderRewriteIterable: IntoIterator<Item = Rewrite<Content>>,
    BodyRewriteIterable: IntoIterator<Item = Rewrite<Content>>,
{
    let mut source_positions_used_in_mappings: Vec<BytePos> = Vec::new();

    // Sort the body rewrites.
    let mut body_rewrites: Vec<Rewrite<Content>> = body_iterable.into_iter().collect();
    body_rewrites.sort_unstable();

    // Filter out rewrites that overlap with previous rewrites, since they would otherwise
    // conflict.
    let mut prev_hi: Option<BytePos> = None;
    let filtered_body_rewrites_iterable = body_rewrites.into_iter().filter(|rewrite| {
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
    });

    // Combine the header and body rewrites.
    let rewrites: Vec<Rewrite<Content>> = header_iterable
        .into_iter()
        .chain(filtered_body_rewrites_iterable)
        .inspect(|rewrite| {
            // Collect positions that we'll need to track to generate source mappings.
            if let Some(pos) = rewrite.content().source_pos() {
                source_positions_used_in_mappings.push(pos);
            }
        })
        .collect();

    source_positions_used_in_mappings.sort_unstable();

    // The result should be a safe rewrite plan.
    RewritePlan {
        rewrites,
        source_positions_used_in_mappings,
    }
}

impl<Content: RewriteContent> RewritePlan<Content> {
    pub fn apply<'a>(
        self: Self,
        input_file: &mut InputFile<'a>,
        token_positions: Vec<BytePos>,
        embed_code_in_source_map: bool,
    ) -> (String, SourceMap) {
        let embedded_code = if embed_code_in_source_map {
            Some(vec![Some(String::from(input_file.code).into())])
        } else {
            None
        };

        let mut output = RewriteOutput::new(
            input_file,
            token_positions,
            self.source_positions_used_in_mappings,
        );

        for rewrite in &self.rewrites {
            match rewrite {
                Rewrite::Replace { content, span } => {
                    output.emit_input_until(span.lo());
                    output.emit_replacement_until(
                        span.hi(),
                        &format!("{}", content),
                        content.source_pos(),
                    );
                }
                Rewrite::Insert { content, pos } => {
                    output.emit_input_until(*pos);
                    output.emit_insertion(&format!("{}", content), content.source_pos());
                }
            }
        }

        let (rewritten_code, source_map_tokens) = output.finish();

        let source_map = SourceMap::new(
            None,
            source_map_tokens,
            vec![],
            vec![String::from(input_file.name).into()],
            embedded_code,
        );

        return (rewritten_code, source_map);
    }
}
