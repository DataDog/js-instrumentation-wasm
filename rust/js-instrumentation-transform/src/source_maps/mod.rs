use std::io::BufWriter;

use anyhow::Result;
use js_instrumentation_shared::debug_log;
use swc_common::Span;
use swc_core::base::sourcemap::SourceMap;

use crate::comments::SourceMapComment;

pub fn parse_source_map(unparsed_map: &[u8]) -> Result<SourceMap> {
    SourceMap::from_reader(unparsed_map)
        .map_err(|err| anyhow::anyhow!("Parsing input source map failed: {}", err))
}

pub fn serialize_source_map(map: SourceMap) -> Result<String> {
    let mut source_map_buffer = BufWriter::new(Vec::new());
    map.to_writer(&mut source_map_buffer)
        .map_err(|err| anyhow::anyhow!("Serializing output source map failed: {}", err))?;
    let source_map_writer = source_map_buffer
        .into_inner()
        .map_err(|err| anyhow::anyhow!("Unwrapping output source map failed: {}", err))?;
    String::from_utf8(source_map_writer)
        .map_err(|err| anyhow::anyhow!("Converting output source map to string failed: {}", err))
}

pub fn chain_source_map_if_needed(
    source_map_comment: &Option<SourceMapComment>,
    input_source_map: &Option<String>,
    transform_source_map: SourceMap,
) -> Result<Option<SourceMap>> {
    match (source_map_comment, input_source_map) {
        // If there's an external source map, we have no way of chaining with it. Generate no
        // source map in this case.
        (Some(SourceMapComment::External()), _) => {
            debug_log("Detected external source map. Will not generate a source map.");
            Ok(None)
        }

        // We got source maps from two different sources! Arbitrarily choose to trust the
        // input source map and chain with it.
        (Some(SourceMapComment::Inline(_, _)), Some(unparsed_map)) => {
            debug_log("Detected an inline source map, but an input source map was provided. Ignoring inline source map.");
            let mut map = parse_source_map(&unparsed_map.as_bytes())?;
            map.adjust_mappings(&transform_source_map);
            Ok(Some(map))
        }

        // We got an inline source map; chain with it.
        (Some(SourceMapComment::Inline(unparsed_map, _)), None) => {
            let mut map = parse_source_map(&unparsed_map)?;
            map.adjust_mappings(&transform_source_map);
            Ok(Some(map))
        }

        // We got an input source map; chain with it.
        (None, Some(unparsed_map)) => {
            let mut map = parse_source_map(&unparsed_map.as_bytes())?;
            map.adjust_mappings(&transform_source_map);
            Ok(Some(map))
        }

        // There's nothing to chain, so just use the transform source map as-is.
        (None, None) => Ok(Some(transform_source_map)),
    }
}

pub fn source_map_comment_span_to_delete(
    source_map_comment: &Option<SourceMapComment>,
) -> Option<Span> {
    match source_map_comment {
        // Don't delete external source map comments.
        Some(SourceMapComment::External()) => None,

        // Always delete inline source map comments, because after the transform runs, the inline
        // source map is wrong.
        Some(SourceMapComment::Inline(_, span)) => Some(*span),

        // If there was no source map comment, we don't need to take any action.
        None => None,
    }
}
