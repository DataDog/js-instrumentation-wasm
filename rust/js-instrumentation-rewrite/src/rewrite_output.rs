use std::collections::HashMap;

use js_instrumentation_shared::InputFile;
use swc_common::{
    input::{Input, StringInput},
    BytePos,
};
use swc_core::base::sourcemap::RawToken;

#[derive(Debug)]
struct LineAndCol {
    line: u32,
    col: u32,
}

enum TokenOrPlaceholder {
    Token(RawToken),
    Placeholder(BytePos, LineAndCol),
}

pub struct RewriteOutput<'a> {
    dst_buffer: String,
    dst_line: u32,
    dst_col: u32,
    src_line: u32,
    src_col: u32,

    input: StringInput<'a>,
    input_end_pos: BytePos,
    source_map_tokens: Vec<TokenOrPlaceholder>,

    next_token_position: BytePos,
    token_positions: Vec<BytePos>,
    token_position_index: usize,

    next_tracked_source_position: BytePos,
    tracked_source_positions: Vec<BytePos>,
    tracked_source_position_index: usize,
    resolved_source_positions: HashMap<BytePos, LineAndCol>,
}

impl<'a> RewriteOutput<'a> {
    pub fn new<'b>(
        input_file: &'a mut InputFile<'b>,
        token_positions: Vec<BytePos>,
        source_positions_used_in_mappings: Vec<BytePos>,
    ) -> Self {
        let token_position_index: usize = 0;
        let next_token_position = token_positions
            .get(token_position_index)
            .map_or(BytePos::DUMMY, |p| *p);

        let tracked_source_position_index: usize = 0;
        let next_tracked_source_position = source_positions_used_in_mappings
            .get(tracked_source_position_index)
            .map_or(BytePos::DUMMY, |p| *p);

        RewriteOutput {
            dst_buffer: String::new(),
            dst_line: 0,
            dst_col: 0,
            src_line: 0,
            src_col: 0,

            input: input_file.as_string_input(),
            input_end_pos: input_file.end_pos,
            source_map_tokens: Vec::new(),

            next_token_position,
            token_positions,
            token_position_index,

            next_tracked_source_position,
            tracked_source_positions: source_positions_used_in_mappings,
            tracked_source_position_index,
            resolved_source_positions: HashMap::new(),
        }
    }

    /// Advance the input file to the given position, emitting all input between the two points.
    pub fn emit_input_until(self: &mut Self, src_end_pos: BytePos) {
        let src_start_pos = self.input.cur_pos();

        while self.input.cur_pos() < src_end_pos {
            self.maybe_emit_source_map_token_for_src_position();

            // Advance by one character if possible.
            match self.input.cur() {
                Some(ch) => {
                    unsafe {
                        self.input.bump();
                    }
                    if ch == '\n' {
                        self.dst_line += 1;
                        self.dst_col = 0;
                        self.src_line += 1;
                        self.src_col = 0;
                    } else {
                        self.dst_col += 1;
                        self.src_col += 1;
                    }
                }
                None => break,
            }
        }

        unsafe {
            self.dst_buffer += self.input.slice(src_start_pos, src_end_pos);
        }
    }

    /// Advance the input file to the given position. The original input is discarded, and the
    /// given replacement string is emitted instead. If a source_pos is given, a source mapping
    /// will be generated pointing to it from the beginning of the replacement string.
    pub fn emit_replacement_until(
        self: &mut Self,
        src_end_pos: BytePos,
        replacement_string: &str,
        source_pos: Option<BytePos>,
    ) {
        while self.input.cur_pos() < src_end_pos {
            self.maybe_emit_source_map_token_for_src_position();

            // Advance by one character if possible.
            match self.input.cur() {
                Some(ch) => {
                    unsafe {
                        self.input.bump();
                    }
                    if ch == '\n' {
                        self.src_line += 1;
                        self.src_col = 0;
                    } else {
                        self.src_col += 1;
                    }
                }
                None => break,
            }
        }

        self.emit_insertion(replacement_string, source_pos);
    }

    /// Emit the given string. The current position in the input file is unaffected.
    /// If a source_pos is given, a source mapping will be generated pointing to it
    /// from the beginning of the inserted string.
    pub fn emit_insertion(self: &mut Self, inserted_string: &str, source_pos: Option<BytePos>) {
        if let Some(pos) = source_pos {
            // We may not know the line or column we need to point to yet, so generate a
            // placeholder instead. After we've processed the entire source, we'll know
            // the line and column for all locations, and we'll replace these placeholders
            // with real mappings.
            self.emit_source_map_token_placeholder(pos)
        }

        for ch in inserted_string.chars() {
            self.maybe_emit_source_map_token_for_dst_position();
            if ch == '\n' {
                self.dst_line += 1;
                self.dst_col = 0;
            } else {
                self.dst_col += 1;
            }
        }

        self.dst_buffer += inserted_string;
    }

    /// Advance to the end of the input, emitting all remaining content. Returns the output string
    /// and the tokens for the output source map.
    pub fn finish(mut self: Self) -> (String, Vec<RawToken>) {
        self.emit_input_until(self.input_end_pos);
        let tokens = Self::evaluate_source_map_token_placeholders(
            self.resolved_source_positions,
            self.source_map_tokens,
        );
        (self.dst_buffer, tokens)
    }

    fn evaluate_source_map_token_placeholders(
        resolved_source_positions: HashMap<BytePos, LineAndCol>,
        tokens_with_placeholders: Vec<TokenOrPlaceholder>,
    ) -> Vec<RawToken> {
        tokens_with_placeholders
            .into_iter()
            .filter_map(|token_or_placeholder| match token_or_placeholder {
                TokenOrPlaceholder::Token(token) => Some(token),
                TokenOrPlaceholder::Placeholder(pos, dst_line_and_col) => {
                    match resolved_source_positions.get(&pos) {
                        Some(src_line_and_col) => Some(RawToken {
                            dst_line: dst_line_and_col.line,
                            dst_col: dst_line_and_col.col,
                            src_line: src_line_and_col.line,
                            src_col: src_line_and_col.col,
                            src_id: 0,
                            name_id: !0,
                            is_range: false,
                        }),
                        _ => None,
                    }
                }
            })
            .collect()
    }

    fn maybe_emit_source_map_token_for_src_position(self: &mut Self) {
        let cur_pos = self.input.cur_pos();

        // If we've reached the next tracked source position, record the line and column
        // it corresponds to, so that we can replace any source mapping placeholders
        // for this position with actual mappings.
        if cur_pos == self.next_tracked_source_position {
            self.resolved_source_positions.insert(
                cur_pos,
                LineAndCol {
                    line: self.src_line,
                    col: self.src_col,
                },
            );
            self.tracked_source_position_index += 1;
            self.next_tracked_source_position = self
                .tracked_source_positions
                .get(self.tracked_source_position_index)
                .map_or(BytePos::DUMMY, |p| *p);
        }

        if cur_pos == self.next_token_position {
            // Emit a mapping because there's a mapped token at this position.
            self.emit_source_map_token();
            self.token_position_index += 1;
            self.next_token_position = self
                .token_positions
                .get(self.token_position_index)
                .map_or(BytePos::DUMMY, |p| *p);
        } else if self.src_col == 0 {
            // Emit a mapping for the first character of every source line. This guarantees that
            // every line receives at least one mapping, and it ensures the minor source map
            // inaccuracies never cause a character to be associated with the previous line instead
            // of the line it's actually on.
            self.emit_source_map_token();
        }
    }

    fn maybe_emit_source_map_token_for_dst_position(self: &mut Self) {
        if self.dst_col == 0 {
            // Emit a mapping for the first character of every destination line. This guarantees
            // that every line receives at least one mapping, and it ensures the minor source map
            // inaccuracies never cause a character to be associated with the previous line instead
            // of the line it's actually on.
            self.emit_source_map_token();
        }
    }

    fn emit_source_map_token(self: &mut Self) {
        self.source_map_tokens
            .push(TokenOrPlaceholder::Token(RawToken {
                dst_line: self.dst_line,
                dst_col: self.dst_col,
                src_line: self.src_line,
                src_col: self.src_col,
                src_id: 0,
                name_id: !0,
                is_range: false,
            }));
    }

    fn emit_source_map_token_placeholder(self: &mut Self, pos: BytePos) {
        self.source_map_tokens.push(TokenOrPlaceholder::Placeholder(
            pos,
            LineAndCol {
                line: self.dst_line,
                col: self.dst_col,
            },
        ))
    }
}
