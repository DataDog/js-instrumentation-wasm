use swc_common::{source_map::SmallPos, BytePos};
use swc_ecma_parser::{Input, StringInput};

use crate::input::InputFile;

use super::EmitError;

#[derive(Clone)]
pub enum OutputScopeKind {
    Invalid,
    Valid,
}

pub struct OutputEmitter<'a> {
    errors: Vec<EmitError>,
    input: StringInput<'a>,
    input_pos: BytePos,
    output: String,
    scopes: Vec<OutputScopeKind>,
}

impl<'a> OutputEmitter<'a> {
    pub fn new(input_file: &InputFile<'a>) -> OutputEmitter<'a> {
        let input = input_file.as_input();
        let input_pos = input.start_pos();
        OutputEmitter {
            errors: Vec::new(),
            input,
            input_pos,
            output: String::new(),
            scopes: Vec::new(),
        }
    }

    pub fn enter_scope(self: &mut Self, kind: OutputScopeKind) {
        self.scopes.push(kind);
    }

    pub fn exit_scope(self: &mut Self) {
        self.scopes.pop();
    }

    fn should_emit_replacements(self: &Self) -> bool {
        match self.scopes.last() {
            Some(OutputScopeKind::Valid) => true,
            Some(OutputScopeKind::Invalid) => false,
            None => true,
        }
    }

    pub fn emit(self: &mut Self, string: &str) {
        if !self.should_emit_replacements() {
            return;
        }
        self.output += string;
    }

    pub fn emit_input_until(self: &mut Self, end: BytePos) {
        self.emit_input_span(self.input_pos, end);
        self.input_pos = end;
    }

    pub fn emit_replacement_until(self: &mut Self, end: BytePos, replacement: &str) {
        if !self.should_emit_replacements() {
            self.emit_input_until(end);
            return;
        }
        self.output += replacement;
        self.input_pos = end;
    }

    pub fn emit_input_or_replacement_until(self: &mut Self, end: BytePos, replacement: &str) {
        let replacement_length = replacement.len();
        let original_length = end.to_usize() - self.input_pos.to_usize();
        if replacement_length < original_length {
            self.emit_replacement_until(end, replacement);
        } else {
            self.emit_input_until(end);
        }
    }

    pub fn emit_rest_of_input(self: &mut Self) {
        self.emit_input_until(self.input.end_pos());
    }

    fn emit_input_span(self: &mut Self, lo: BytePos, hi: BytePos) {
        unsafe {
            self.output += self.input.slice(lo, hi);
        }
    }

    pub fn report_error(self: &mut Self, error: EmitError) {
        self.errors.push(error);
    }

    pub fn take(self: Self) -> (String, Vec<EmitError>) {
        (self.output, self.errors)
    }
}
