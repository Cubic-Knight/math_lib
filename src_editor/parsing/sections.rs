use std::collections::HashMap;
use super::{
    FileLine, ProvenState,
    parse_new_syntax,
    parse_formula
};
use crate::graphics::IndentInfo;
use crate::library_data::{
    LibraryData, Reference,
    Syntax, SyntaxType
};

pub fn parse_syntax_section(section: Vec<&str>, syntax_type: SyntaxType) -> (Vec<FileLine>, Option<Syntax>) {
    let mut lines = section.into_iter();

    let section_name_line = match lines.next() {
        Some(section_name) => FileLine::Section {
            name: section_name.to_owned(),
            is_valid: section_name == "# Syntax"
        },
        None => return (vec![], None)
    };
    let (syntax, syntax_line) = match lines.next() {
        Some(line) => {
            let (colored_string, syntax) = parse_new_syntax(line, syntax_type);
            let assertion = FileLine::Assertion {
                assertion: colored_string,
                is_proven: ProvenState::None
            };
            (syntax, assertion)
        },
        None => return (vec![ section_name_line ], None)
    };
    
    let mut result_lines = vec![ section_name_line, syntax_line ];
    for line in lines {
        result_lines.push( FileLine::UnexpectedLine(line.to_owned()) );
    };
    (result_lines, syntax)
}

pub fn parse_definition_section(
    section: Vec<&str>, lib_data: &LibraryData, new_syntax: Option<Syntax>
) -> Vec<FileLine> {
    let mut lines = section.into_iter();

    let section_name_line = match lines.next() {
        Some(section_name) => FileLine::Section {
            name: section_name.to_owned(),
            is_valid: section_name == "# Definition"
        },
        None => return vec![]
    };
    let definition_line = match lines.next() {
        Some(line) => {
            let colored_string = parse_formula(line, lib_data, new_syntax);
            let assertion = FileLine::Assertion {
                assertion: colored_string,
                is_proven: ProvenState::None
            };
            assertion
        },
        None => return vec![ section_name_line ]
    };
    
    let mut result_lines = vec![ section_name_line, definition_line ];
    for line in lines {
        result_lines.push( FileLine::UnexpectedLine(line.to_owned()) );
    };
    result_lines
}

pub fn parse_hypotesis_section(section: Vec<&str>, lib_data: &LibraryData) -> Vec<FileLine> {
    let mut lines = section.into_iter();

    let section_name_line = match lines.next() {
        Some(section_name) => FileLine::Section {
            name: section_name.to_owned(),
            is_valid: section_name == "# Hypothesis" || section_name == "# Hypotheses"
        },
        None => return vec![]
    };

    let mut result_lines = vec![ section_name_line ];
    for line in lines {
        let hypothesis = match line.split_once(':') {
            Some((name, hypot)) => FileLine::Hypothesis {
                name: name.to_owned(),
                hypot: parse_formula(hypot, lib_data, None)
            },
            None => FileLine::UnexpectedLine(line.to_owned()) 
        };
        result_lines.push( hypothesis );
    };
    result_lines
}

pub fn parse_assertion_section(section: Vec<&str>, lib_data: &LibraryData) -> Vec<FileLine> {
    let mut lines = section.into_iter();

    let section_name_line = match lines.next() {
        Some(section_name) => FileLine::Section {
            name: section_name.to_owned(),
            is_valid: section_name == "# Assertion" || section_name == "# Assertions"
        },
        None => return vec![]
    };

    let mut result_lines = vec![ section_name_line ];
    for line in lines {
        let assertion = FileLine::Assertion {
            assertion: parse_formula(line, lib_data, None),
            is_proven: ProvenState::NotProven
        };
        result_lines.push( assertion );
    };
    result_lines
}

pub fn parse_proof_section(
    section: Vec<&str>, lib_data: &LibraryData, references: &HashMap<String, Reference>
) -> (Vec<FileLine>, IndentInfo) {
    let mut indent_info = IndentInfo {
        line_number_indent: 4,
        used_hypotheses_indent: 2,
        theorem_reference_indent: 2
    };

    let mut lines = section.into_iter();

    let section_name_line = match lines.next() {
        Some(section_name) => FileLine::Section {
            name: section_name.to_owned(),
            is_valid: section_name == "# Proof"
        },
        None => return (vec![], indent_info)
    };

    let mut result_lines = vec![ section_name_line ];
    for (i, line) in lines.enumerate() {
        let mut parts = line.splitn(4, ';')
            .map(|s| s.trim());
        let line_no = parts.next().unwrap_or("").to_owned();
        if line_no.len() > indent_info.line_number_indent {
            indent_info.line_number_indent = match line_no.len() % 4 {
                0 => line_no.len(),
                r => line_no.len() + (4 - r)
            };
        };
        let used_hypots = parts.next().unwrap_or("")
            .split(',').map(|s| s.trim().to_owned())
            .collect::<Vec<_>>();
        let used_hypots_len = used_hypots.iter()
            .map(|s| s.len())
            .sum::<usize>() + 2 * (used_hypots.len() - 1);
        if used_hypots_len > indent_info.used_hypotheses_indent {
            indent_info.used_hypotheses_indent = match (used_hypots_len - 2) % 4 {
                0 => used_hypots_len,
                r => used_hypots_len + (4 - r)
            };
        };
        let theo_ref = parts.next().unwrap_or("").to_owned();
        if theo_ref.len() > indent_info.theorem_reference_indent {
            indent_info.theorem_reference_indent = match (theo_ref.len() - 2) % 4 {
                0 => theo_ref.len(),
                r => theo_ref.len() + (4 - r)
            };
        };
        let resulting_formula = parts.next().unwrap_or("");

        let proof_line = FileLine::ProofLine {
            line_no, line_index: i+1,
            used_hypots,
            theo_ref_exists: references.contains_key(&theo_ref),
            theo_ref,
            formula: parse_formula(resulting_formula, lib_data, None)
        };
        result_lines.push( proof_line );
    };
    (result_lines, indent_info)
}
