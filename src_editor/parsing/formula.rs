use std::collections::HashMap;
use super::{ColoredString, Color};
use crate::library_data::{
    Syntax, SyntaxType,
    Placeholder,
    LibraryData
};

pub fn parse_new_syntax(line: &str, syntax_type: SyntaxType) -> (ColoredString, Option<Syntax>) {
    if line.len() == 0 {
        return (
            ColoredString { characters: vec![], colors: vec![] },
            None
        );
    };
    let mut characters = Vec::new();
    let mut colors = Vec::new();
    let mut formula = Vec::new();
    let mut wff_mapping = HashMap::new();
    let mut obj_mapping = HashMap::new();
    for c in line.chars() {
        if c == ' ' {
            characters.push(c);
            colors.push(Color::Normal);
            continue;
        };
        if c == '…' {
            characters.push(c);
            colors.push(Color::Black);
            formula.push(Placeholder::Repetition);
        } else if '𝑎' <= c && c <= '𝑧' {  // '𝑎' and '𝑧' here are NOT ascii
            characters.push(c);
            colors.push(Color::Red);
            match wff_mapping.get(&c) {
                Some(id) => formula.push(Placeholder::WellFormedFormula(*id)),
                None => {
                    formula.push(Placeholder::WellFormedFormula(wff_mapping.len()));
                    wff_mapping.insert(c, wff_mapping.len());
                }
            };
        } else if '𝛼' <= c && c <= '𝜔' {
            characters.push(c);
            colors.push(Color::Blue);
            match obj_mapping.get(&c) {
                Some(id) => formula.push(Placeholder::Object(*id)),
                None => {
                    formula.push(Placeholder::Object(obj_mapping.len()));
                    obj_mapping.insert(c, obj_mapping.len());
                }
            };
        } else {
            characters.push(c);
            colors.push(Color::White);
            formula.push(Placeholder::LiteralChar(c))
        };
    };
    let syntax = Syntax {
        syntax_type,
        formula,
        distinct_wff_count: wff_mapping.len(),
        distinct_object_count: obj_mapping.len()
    };
    (ColoredString { characters, colors }, Some(syntax))
}

#[derive(Debug)]
enum PartiallyCompiled {
    NotCompiled(char),
    Space,
    CompiledFormula { characters: Vec<char>, colors: Vec<Color> },
    CompiledObject { characters: Vec<char>, colors: Vec<Color> }
}

fn monochromatic_formula(line: &str, color: Color) -> ColoredString {
    ColoredString {
        characters: line.chars().collect(),
        colors: line.chars().map(|_| color).collect()
    }
}

pub fn parse_formula<'a>(
    line: &'a str, lib_data: &LibraryData, additional_syntax: Option<Syntax>
) -> ColoredString {
    let leading_spaces_count = line.len() - line.trim_start().len();
    let trailing_spaces_count = line.len() - line.trim_end().len();
    let try_parsing = line.trim().chars()
        .map(|ch| match ch {
            '…' => Err(()),
            ' ' => Ok(PartiallyCompiled::Space),
            c @ '𝑎'..='𝑧' => Ok(PartiallyCompiled::CompiledObject {
                characters: vec![ c ], colors: vec![ Color::Red ]
            }),
            c @ '𝛼'..='𝜔' => Ok(PartiallyCompiled::CompiledFormula {
                characters: vec![ c ], colors: vec![ Color::Blue ]
            }),
            c => Ok(PartiallyCompiled::NotCompiled(c))
        })
        .collect::<Result<Vec<_>, _>>();
    let mut partially_compiled = match try_parsing {
        Ok(list) => list,
        Err(()) => return monochromatic_formula(line, Color::Red)
    };
    let syntaxes = additional_syntax.iter()
        .chain(lib_data.syntaxes.iter())
        .collect::<Vec<_>>();
    'try_find_patterns: while partially_compiled.len() > 1 {
        'try_syntax: for (syntax_id, syntax) in syntaxes.iter().enumerate() {
            'try_position: for index in 0..partially_compiled.len() {
                if partially_compiled.len() - index < syntax.formula.len() {
                    continue 'try_syntax;  // If the syntax does not fit in the rest of the text, go to the next syntax
                };
                if let Some(PartiallyCompiled::Space) = partially_compiled.get(index) {
                    continue 'try_position;  // We ignore spaces so we skip any leading space
                };
                let syntax_color = match (
                    &syntax.syntax_type, syntax.distinct_wff_count, syntax.distinct_object_count
                ) {
                    (_, _, _) if syntax_id == 0 && additional_syntax.is_some() => Color::White,
                    (SyntaxType::Formula, 0, 0) => Color::Green,
                    (SyntaxType::Formula, _, _) => Color::Cyan,
                    (SyntaxType::Object, 0, 0) => Color::Yellow,
                    (SyntaxType::Object, _, _) => Color::Magenta
                };

                let mut wffs = vec![None; syntax.distinct_wff_count];
                let mut objects = vec![None; syntax.distinct_object_count];
                
                let mut i = index;
                let mut characters = Vec::new();
                let mut colors = Vec::new();
                let mut syntax_length = 0;
                for pl in &syntax.formula {
                    let c = loop {
                        match partially_compiled.get(i) {
                            Some(PartiallyCompiled::Space) => {
                                characters.push(' ');
                                colors.push(Color::Normal);
                                syntax_length += 1;
                                i += 1;
                            },
                            Some(chr) => break chr,
                            None => continue 'try_syntax
                        };
                    };
                    let valid = match (c, pl) {
                        (
                            PartiallyCompiled::NotCompiled(c1),
                            Placeholder::LiteralChar(c2)
                        ) => {
                            characters.push(*c1);
                            colors.push(syntax_color);
                            syntax_length += 1;
                            c1 == c2
                        },
                        (
                            PartiallyCompiled::CompiledFormula {
                                characters: chs, colors: cols
                            },
                            Placeholder::WellFormedFormula(id)
                        ) => {
                            characters.extend(chs);
                            colors.extend(cols);
                            syntax_length += 1;
                            match &wffs[*id] {
                                Some(chrs) => chs == *chrs,
                                None => { wffs[*id] = Some(chs); true }
                            }
                        },
                        (
                            PartiallyCompiled::CompiledObject {
                                characters: chs, colors: cols
                            },
                            Placeholder::Object(id)
                        ) => {
                            characters.extend(chs);
                            colors.extend(cols);
                            syntax_length += 1;
                            match &objects[*id] {
                                Some(chrs) => chs == *chrs,
                                None => { objects[*id] = Some(chs); true }
                            }
                        },
                        _ => false
                    };
                    match valid {
                        true => (),
                        false => continue 'try_position  // Syntax did not match here, try next position
                    };
                    i += 1;
                };
                for _ in 0..syntax_length {
                    partially_compiled.remove(index);
                };
                let element_to_insert = match syntax.syntax_type {
                    SyntaxType::Formula => PartiallyCompiled::CompiledFormula {
                        characters, colors
                    },
                    SyntaxType::Object => PartiallyCompiled::CompiledObject {
                        characters, colors
                    }
                };
                partially_compiled.insert(index, element_to_insert);
                // Once a syntax has been compiled, we iterate through the list again from the beginning
                continue 'try_find_patterns;
            };
        };
        // We only can get here if no syntax has matched
        return monochromatic_formula(line, Color::Red);
    };

    match partially_compiled.pop() {
        Some(
            PartiallyCompiled::CompiledFormula { characters, colors }
        ) => ColoredString {
            characters: vec![' '; leading_spaces_count].into_iter()
                .chain(characters)
                .chain(vec![' '; trailing_spaces_count])
                .collect(),
            colors: vec![Color::Normal; leading_spaces_count].into_iter()
            .chain(colors)
            .chain(vec![Color::Normal; trailing_spaces_count])
            .collect()
        },
        _ => monochromatic_formula(line, Color::Red)
    }
}
