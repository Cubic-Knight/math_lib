use std::{
    fs, io,
    collections::HashMap
};

use super::{
    FileLine, FileType,
    parse_syntax_section,
    parse_definition_section,
    parse_hypotesis_section,
    parse_assertion_section,
    parse_proof_section
};
use crate::graphics::IndentInfo;
use crate::library_data::{
    LibraryData, Reference, SyntaxType
};

pub fn parse_title(lines: &mut std::str::Lines) -> (FileLine, FileType) {
    let first_line = match lines.next() {
        None => return (FileLine::Raw("".to_string()), FileType::Unknown),
        Some(line) => line
    };
    if !first_line.starts_with("##") {
        return (FileLine::Raw(first_line.to_string()), FileType::Unknown);
    }
    let (title, name) = match first_line.rsplit_once(' ') {
        None => return (FileLine::Raw(first_line.to_string()), FileType::Unknown),
        Some((title, name)) => (title, name)
    };
    let (title_good, file_type) = match title {
        "## Syntax Definition (formula)" => (true, FileType::SyntaxDefinitionFormula),
        "## Syntax Definition (object)" => (true, FileType::SyntaxDefinitionObject),
        "## Axiom" => (true, FileType::Axiom),
        "## Theorem" => (true, FileType::Theorem),
        _ => (false, FileType::Unknown)
    };
    let name_good = name.chars().all(|c| c.is_ascii_alphanumeric());
    let title = title.to_owned();
    let name = name.to_owned();
    ( FileLine::Title { title, name, title_good, name_good }, file_type )
}

pub fn parse_file(
    path: String, lib_data: &LibraryData, references: &HashMap<String, Reference>
) -> io::Result<(Vec<FileLine>, IndentInfo)> {
    let contents = fs::read_to_string(path)?;
    let mut lines = contents.lines();

    let (title, file_type) = parse_title(&mut lines);
    
    let mut sections = Vec::new();
    let mut temp = Vec::new();
    for line in lines {
        if line.starts_with('#') {
            sections.push(temp);
            temp = Vec::new();
        };
        temp.push(line);
    };
    sections.push(temp);
    let mut sections = sections.into_iter();

    let mut indent_info = IndentInfo {
        line_number_indent: 0,
        used_hypotheses_indent: 0,
        theorem_reference_indent: 0
    };
    let mut result_lines = vec![ title ];
    if let Some(empty_first_section) = sections.next() {
        for line in empty_first_section {
            result_lines.push( FileLine::Raw(line.to_owned()) );
        };
    };
    match file_type {
        FileType::SyntaxDefinitionFormula => {
            let Some(syntax_section) = sections.next() else {
                return Ok((result_lines, indent_info));
            };
            let (
                mut syntax_lines, new_syntax
            ) = parse_syntax_section(syntax_section, SyntaxType::Formula);
            result_lines.append( &mut syntax_lines );
            if let Some(definition_section) = sections.next() {
                result_lines.append(
                    &mut parse_definition_section(definition_section, lib_data, new_syntax)
                );
            };
        },
        FileType::SyntaxDefinitionObject => {
            let Some(syntax_section) = sections.next() else {
                return Ok((result_lines, indent_info));
            };
            let (
                mut syntax_lines, new_syntax
            ) = parse_syntax_section(syntax_section, SyntaxType::Object);
            result_lines.append( &mut syntax_lines );
            if let Some(definition_section) = sections.next() {
                result_lines.append(
                    &mut parse_definition_section(definition_section, lib_data, new_syntax)
                );
            };
        },
        FileType::Axiom => {
            if let Some(hypothesis_section) = sections.next() {
                result_lines.append(
                    // Hypotheses are not named in Axioms,
                    // So they are parsed as if they were assertions
                    &mut parse_assertion_section(hypothesis_section, lib_data)
                );
            };
            if let Some(assertion_section) = sections.next() {
                result_lines.append(
                    &mut parse_assertion_section(assertion_section, lib_data)
                );
            };
        },
        FileType::Theorem => {
            if let Some(hypothesis_section) = sections.next() {
                result_lines.append(
                    &mut parse_hypotesis_section(hypothesis_section, lib_data)
                );
            };
            if let Some(assertion_section) = sections.next() {
                result_lines.append(
                    &mut parse_assertion_section(assertion_section, lib_data)
                );
            };
            if let Some(proof_section) = sections.next() {
                let (
                    mut proof_lines, indents
                ) = parse_proof_section(proof_section, lib_data, references);
                indent_info = indents;
                result_lines.append( &mut proof_lines );
            };
        },
        FileType::Unknown => {
            for lines in sections.by_ref() {
                for line in lines {
                    result_lines.push( FileLine::Raw(line.to_owned()) );
                };
            };
        }
    }
    for lines in sections {
        for line in lines {
            result_lines.push( FileLine::UnexpectedLine(line.to_owned()) );
        };
    };

    Ok((result_lines, indent_info))
}
