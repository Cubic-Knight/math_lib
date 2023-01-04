use std::collections::HashMap;

use crate::parsing::{
    parse_file, FileLine
};
use crate::library_data::{
    LibraryData, Reference
};

#[derive(Default)]
pub struct FileGraphics {
    pub cursor: (usize, usize),
    pub lines: Vec<FileLine>,
    pub indent_info: IndentInfo,
    pub read_only: bool
}

#[derive(Default)]
pub struct IndentInfo {
    pub line_number_indent: usize,
    pub used_hypotheses_indent: usize,
    pub theorem_reference_indent: usize
}

pub fn get_file(
    path: String, lib_data: &LibraryData, references: &HashMap<String, Reference>
) -> Result<FileGraphics, ()> {
    let (
        file_lines, indent_info
    ) = parse_file(path, lib_data, references).map_err(|_| ())?;

    Ok(
        FileGraphics {
            cursor: (1, 1),
            lines: file_lines,
            indent_info,
            read_only: true
        }
    )
}
