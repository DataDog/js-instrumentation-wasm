use swc_common::{BytePos, FileName, FilePathMapping, SourceMap};
use swc_ecma_parser::StringInput;

pub struct InputFile<'a> {
    pub code: &'a str,
    pub name: &'a str,
    pub start_pos: BytePos,
    pub end_pos: BytePos,
}

impl<'a> InputFile<'a> {
    pub fn new(name: &'a str, code: &'a str) -> InputFile<'a> {
        let map = SourceMap::new(FilePathMapping::empty());
        let file = map.new_source_file(FileName::Custom(name.to_string()).into(), code.to_string());
        let start_pos = file.start_pos;
        let end_pos = file.end_pos;

        InputFile {
            code,
            name,
            start_pos,
            end_pos,
        }
    }

    pub fn as_input(self: &Self) -> StringInput<'a> {
        StringInput::new(self.code, self.start_pos, self.end_pos)
    }
}
