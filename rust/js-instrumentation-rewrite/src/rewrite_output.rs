use js_instrumentation_shared::InputFile;
use swc_common::{BytePos, Span};
use swc_core::base::sourcemap::RawToken;

pub struct RewriteOutput<'a, 'b> {
    dst_buffer: String,
    dst_line: u32,
    dst_col: u32,
    src_line: u32,
    src_col: u32,
    src_pos: BytePos,

    input_file: &'a mut InputFile<'b>,
    source_map_tokens: Vec<RawToken>,
}

impl<'a, 'b> RewriteOutput<'a, 'b> {
    pub fn new(input_file: &'a mut InputFile<'b>) -> Self {
        let src_pos = input_file.start_pos;
        RewriteOutput {
            dst_buffer: String::new(),
            dst_line: 0,
            dst_col: 0,
            src_line: 0,
            src_col: 0,
            src_pos,

            input_file,
            source_map_tokens: Vec::new(),
        }
    }

    /// Advance the input file to the given position, emitting all input between the two points.
    pub fn emit_input_until(self: &mut Self, src_end_pos: BytePos) {
        if let Some(consumed_span) = self.consume_and_map_span_ending_at(src_end_pos) {
            let original_string = self.input_file.slice_span(consumed_span);
            for ch in original_string.chars() {
                if ch == '\n' {
                    self.dst_line += 1;
                    self.dst_col = 0;
                    self.src_line += 1;
                    self.src_col = 0;
                    continue;
                }
                self.dst_col += 1;
                self.src_col += 1;
            }
            self.dst_buffer += original_string;
        }
    }

    /// Advance the input file to the given position. The original input is discarded, and the
    /// given replacement string is emitted instead.
    pub fn emit_replacement_until(self: &mut Self, src_end_pos: BytePos, replacement_string: &str) {
        if let Some(consumed_span) = self.consume_and_map_span_ending_at(src_end_pos) {
            for ch in self.input_file.slice_span(consumed_span).chars() {
                if ch == '\n' {
                    self.src_line += 1;
                    self.src_col = 0;
                    continue;
                }
                self.src_col += 1;
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
                continue;
            }
            self.dst_col += 1;
        }

        self.dst_buffer += inserted_string;
    }

    /// Advance to the end of the input, emitting all remaining content. Returns the output string
    /// and the tokens for the output source map.
    pub fn finish(mut self: Self) -> (String, Vec<RawToken>) {
        self.emit_input_until(self.input_file.end_pos);
        (self.dst_buffer, self.source_map_tokens)
    }

    /// Consume a span of the input file, starting at the current input position and ending at the
    /// provided position. If the span is non-empty, a source map token is created for it, the
    /// current position is moved to the end of the span, and the span is returned for further
    /// processing. If the span is empty, None is returned.
    fn consume_and_map_span_ending_at(self: &mut Self, src_end_pos: BytePos) -> Option<Span> {
        if self.src_pos == src_end_pos {
            return None;
        }

        self.source_map_tokens.push(RawToken {
            dst_line: self.dst_line,
            dst_col: self.dst_col,
            src_line: self.src_line,
            src_col: self.src_col,
            src_id: 0,
            name_id: !0,
            is_range: true,
        });

        let consumed_span = Span {
            lo: self.src_pos,
            hi: src_end_pos,
        };
        self.src_pos = src_end_pos;
        Some(consumed_span)
    }
}
