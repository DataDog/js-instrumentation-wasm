use swc_common::{BytePos, FileName, FilePathMapping, SourceMap};
use swc_ecma_parser::{Input, StringInput};

pub struct InputFile<'a> {
    pub code: &'a str,
    pub name: &'a str,
    pub start_pos: BytePos,
    pub end_pos: BytePos,
    input: StringInput<'a>,
}

impl<'a> InputFile<'a> {
    pub fn new(name: &'a str, code: &'a str) -> InputFile<'a> {
        let map = SourceMap::new(FilePathMapping::empty());
        let file = map.new_source_file(FileName::Custom(name.to_string()).into(), code.to_string());
        let start_pos = file.start_pos;
        let end_pos = file.end_pos;
        let input = StringInput::new(code, start_pos, end_pos);

        InputFile {
            code,
            name,
            start_pos,
            end_pos,
            input,
        }
    }

    pub fn may_follow_keyword(self: &mut Self, pos: BytePos) -> bool {
        if pos == self.start_pos {
            return false;
        }
        unsafe {
            let prev_byte = self.input.slice(pos - BytePos(1), pos).as_bytes()[0];
            prev_byte >= b'a' && prev_byte <= b'z'
        }
    }

    pub fn slice(self: &mut Self, start: BytePos, end: BytePos) -> &str {
        unsafe { self.input.slice(start, end) }
    }
}
