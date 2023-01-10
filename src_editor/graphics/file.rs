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
    pub camera: usize,
    pub lines: Vec<FileLine>,
    pub read_only: bool
}

pub fn get_file(
    path: String, lib_data: &LibraryData, references: &HashMap<String, Reference>
) -> Result<FileGraphics, ()> {
    let file_lines = parse_file(path, lib_data, references).map_err(|_| ())?;

    Ok(
        FileGraphics {
            cursor: (1, 1),
            camera: 1,
            lines: file_lines,
            read_only: true
        }
    )
}
