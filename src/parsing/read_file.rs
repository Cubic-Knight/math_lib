use super::{
    MathFile, DefinitionType,
    FileType, FileSection,
    parse_formula, parse_named_formula, parse_proof_line
};

#[derive(Debug)]
pub enum ParseError {
    InvalidHeader(String),
    EmptyFile,
    InvalidSection(String, FileType),
    FileWithoutSection,
    UncorrelatedSection(FileSection, FileType),
    MultilineSection(FileSection),
    EmptySection(FileSection),
    InvalidSectionOrder,
    InvalidNamedHypothesis,
    InvalidProofLine,
    InvalidName(String)
}

pub fn parse_file(content: String) -> Result<MathFile, ParseError> {
    let mut lines = content.lines();
    let (file_type, name) = match lines.next() {
        Some(header) => match header.split(' ').collect::<Vec<&str>>()[..] {
            ["##", "Syntax", "Definition", "(formula)", name] => {
                (FileType::FormulaSyntaxDefinition, name.to_owned())
            },
            ["##", "Syntax", "Definition", "(setvar)", name] => {
                (FileType::SetVariableSyntaxDefinition, name.to_owned())
            },
            ["##", "Axiom", name] => (FileType::Axiom, name.to_owned()),
            ["##", "Theorem", name] => (FileType::Theorem, name.to_owned()),
            _ => return Err(ParseError::InvalidHeader(header.to_owned()))
        },
        None => return Err(ParseError::EmptyFile)
    };
    if !name.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(ParseError::InvalidName(name));
    };
    let mut file_contents = Vec::new();
    let mut section_contents = Vec::new();
    let mut section = FileSection::None;
    for line in lines {
        match line {
            "" => (),
            line if line.starts_with('#') => {
                match section {
                    FileSection::None => (),
                    section => file_contents.push( (section, section_contents) )
                };
                section_contents = Vec::new();
                section = match line {
                    "# Syntax" => FileSection::Syntax,
                    "# Definition" => FileSection::Definition,
                    "# Hypothesis" => FileSection::HypothesisList,
                    "# Hypotheses" => FileSection::HypothesisList,
                    "# Assertion" => FileSection::AssertionList,
                    "# Assertions" => FileSection::AssertionList,
                    "# Proof" => FileSection::Proof,
                    line => {
                        return Err(ParseError::InvalidSection(line.to_owned(), file_type))
                    }
                };
            },
            line => section_contents.push(line)
        }
    };
    match section {
        FileSection::None => (),
        section => file_contents.push( (section, section_contents) )
    };

    match file_type {
        FileType::FormulaSyntaxDefinition => {
            match &file_contents[..] {
                [
                    (FileSection::Syntax, syntax_lines)
                ] => {
                    match syntax_lines.len() {
                        0 => return Err(ParseError::EmptySection(FileSection::Syntax)),
                        1 => (),
                        _ => return Err(ParseError::MultilineSection(FileSection::Syntax))
                    };
                    return Ok(
                        MathFile::SyntaxDefinition {
                            name,
                            definition_type: DefinitionType::Formula,
                            syntax: parse_formula(syntax_lines[0]),
                            definition: None
                        }
                    );
                },
                [
                    (FileSection::Syntax, syntax_lines),
                    (FileSection::Definition, definition_lines)
                ] => {
                    match syntax_lines.len() {
                        0 => return Err(ParseError::EmptySection(FileSection::Syntax)),
                        1 => (),
                        _ => return Err(ParseError::MultilineSection(FileSection::Syntax))
                    };
                    match definition_lines.len() {
                        0 => return Err(ParseError::EmptySection(FileSection::Definition)),
                        1 => (),
                        _ => return Err(ParseError::MultilineSection(FileSection::Definition))
                    };
                    return Ok(
                        MathFile::SyntaxDefinition {
                            name,
                            definition_type: DefinitionType::Formula,
                            syntax: parse_formula(syntax_lines[0]),
                            definition: Some(parse_formula(definition_lines[0]))
                        }
                    );
                },
                _ => return Err(ParseError::InvalidSectionOrder)
            };
        },
        FileType::SetVariableSyntaxDefinition => {
            match &file_contents[..] {
                [
                    (FileSection::Syntax, syntax_lines)
                ] => {
                    match syntax_lines.len() {
                        0 => return Err(ParseError::EmptySection(FileSection::Syntax)),
                        1 => (),
                        _ => return Err(ParseError::MultilineSection(FileSection::Syntax))
                    };
                    return Ok(
                        MathFile::SyntaxDefinition {
                            name,
                            definition_type: DefinitionType::SetVar,
                            syntax: parse_formula(syntax_lines[0]),
                            definition: None
                        }
                    );
                },
                [
                    (FileSection::Syntax, syntax_lines),
                    (FileSection::Definition, definition_lines)
                ] => {
                    match syntax_lines.len() {
                        0 => return Err(ParseError::EmptySection(FileSection::Syntax)),
                        1 => (),
                        _ => return Err(ParseError::MultilineSection(FileSection::Syntax))
                    };
                    match definition_lines.len() {
                        0 => return Err(ParseError::EmptySection(FileSection::Definition)),
                        1 => (),
                        _ => return Err(ParseError::MultilineSection(FileSection::Definition))
                    };
                    return Ok(
                        MathFile::SyntaxDefinition {
                            name,
                            definition_type: DefinitionType::SetVar,
                            syntax: parse_formula(syntax_lines[0]),
                            definition: Some(parse_formula(definition_lines[0]))
                        }
                    );
                },
                _ => return Err(ParseError::InvalidSectionOrder)
            };
        },
        FileType::Axiom => {
            match &file_contents[..] {
                [
                    (FileSection::HypothesisList, hypots),
                    (FileSection::AssertionList, asserts)
                ] => {
                    let hypotheses = hypots.into_iter()
                        .map(|fm| parse_formula(fm))
                        .collect();
                    let assertions = asserts.into_iter()
                        .map(|fm| parse_formula(fm))
                        .collect();
                    return Ok( MathFile::Axiom { name, hypotheses, assertions } );
                },
                _ => return Err(ParseError::InvalidSectionOrder)
            }
        },
        FileType::Theorem => {
            match &file_contents[..] {
                [
                    (FileSection::HypothesisList, hypots),
                    (FileSection::AssertionList, asserts),
                    (FileSection::Proof, proof_lines)
                ] => {
                    let try_hypotheses = hypots.into_iter()
                        .map(|fm| parse_named_formula(fm))
                        .collect();
                    let hypotheses = match try_hypotheses {
                        Ok(hypotheses) => hypotheses,
                        Err(()) => return Err(ParseError::InvalidNamedHypothesis)
                    };
                    let assertions = asserts.into_iter()
                        .map(|fm| parse_formula(fm))
                        .collect();
                    let try_proof = proof_lines.into_iter()
                        .map(|prline| parse_proof_line(prline))
                        .collect();
                    let proof = match try_proof {
                        Ok(proof) => proof,
                        Err(()) => return Err(ParseError::InvalidProofLine)
                    };
                    return Ok( MathFile::Theorem { name, hypotheses, assertions, proof } );
                },
                _ => return Err(ParseError::InvalidSectionOrder)
            }
        }
    };

    Err(ParseError::EmptyFile)
}
