use js_instrumentation_shared::InputFile;
use swc_common::{
    input::{Input, StringInput},
    BytePos,
};
use swc_core::base::sourcemap::RawToken;

pub struct RewriteOutput<'a> {
    dst_buffer: String,
    dst_line: u32,
    dst_col: u32,
    src_line: u32,
    src_col: u32,

    input: StringInput<'a>,
    input_end_pos: BytePos,
    source_map_tokens: Vec<RawToken>,

    next_token_position: BytePos,
    token_positions: Vec<BytePos>,
    token_position_index: usize,
}

impl<'a> RewriteOutput<'a> {
    pub fn new<'b>(input_file: &'a mut InputFile<'b>, token_positions: Vec<BytePos>) -> Self {
        let token_position_index: usize = 0;
        let next_token_position = token_positions
            .get(token_position_index)
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
        }
    }

    /// Advance the input file to the given position, emitting all input between the two points.
    pub fn emit_input_until(self: &mut Self, src_end_pos: BytePos) {
        let src_start_pos = self.input.cur_pos();

        while self.input.cur_pos() < src_end_pos {
            self.emit_source_map_token_if_mapping_exists();

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
    /// given replacement string is emitted instead.
    pub fn emit_replacement_until(self: &mut Self, src_end_pos: BytePos, replacement_string: &str) {
        while self.input.cur_pos() < src_end_pos {
            self.emit_source_map_token_if_mapping_exists();

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

        self.emit_insertion(replacement_string);
    }

    /// Emit the given string. The current position in the input file is unaffected.
    pub fn emit_insertion(self: &mut Self, inserted_string: &str) {
        for ch in inserted_string.chars() {
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
        (self.dst_buffer, self.source_map_tokens)
    }

    fn emit_source_map_token_if_mapping_exists(self: &mut Self) {
        if self.input.cur_pos() == self.next_token_position {
            // Emit a mapping because there's a mapped token at this position.
            self.emit_source_map_token();
            self.token_position_index += 1;
            self.next_token_position = self
                .token_positions
                .get(self.token_position_index)
                .map_or(BytePos::DUMMY, |p| *p);
        } else if self.src_col == 0 {
            // Emit a mapping for the first character of every line. This guarantees that every
            // line receives at least one mapping, and it ensures the minor source map inaccuracies
            // never cause a character to be associated with the previous line instead of the line
            // it's actually on.
            self.emit_source_map_token();
        }
    }

    fn emit_source_map_token(self: &mut Self) {
        self.source_map_tokens.push(RawToken {
            dst_line: self.dst_line,
            dst_col: self.dst_col,
            src_line: self.src_line,
            src_col: self.src_col,
            src_id: 0,
            name_id: !0,
            is_range: false,
        });
    }
}
