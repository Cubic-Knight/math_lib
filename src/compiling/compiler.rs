use std::{fs, io, collections::HashMap};
use crate::parsing::parse_file;
use super::{
    Reference,
    compile_syntax, compile_definition, compile_axiom, compile_theorem,
    CompileError,
};

fn get_file_contents(dir: &mut String, filepath: &str) -> io::Result<String> {
    let initial_length = dir.len();
    dir.push_str(filepath);
    let contents = fs::read_to_string(&dir)?;
    dir.truncate(initial_length);
    Ok(contents)
}

enum ComplierState {
    CompilingSyntaxes,
    CompilingAxioms,
    CompilingTheorems,
    Waiting
}

pub fn compile(mut dir: String) -> Result<(), CompileError> {
    let mut syntaxes = Vec::new();
    let mut definitions = Vec::new();
    let mut axioms = Vec::new();
    let mut theorems = Vec::new();
    let mut references = HashMap::new();
    let mut state = ComplierState::Waiting;
    let order = match get_file_contents(&mut dir, "/order.txt") {
        Ok(file) => file,
        Err(_) => return Err(CompileError::OrderFileNotFound)
    };
    for line in order.lines() {
        if line == "" { continue; };
        if line.starts_with('#') {
            match (state, line) {
                (ComplierState::Waiting, "# Syntax Definitions") => {
                    state = ComplierState::CompilingSyntaxes;
                    continue;
                },
                (ComplierState::CompilingSyntaxes, "# Axioms") => {
                    state = ComplierState::CompilingAxioms;
                    continue;
                },
                (ComplierState::CompilingAxioms, "# Theorems") => {
                    state = ComplierState::CompilingTheorems;
                    continue;
                },
                _ => return Err(CompileError::InvalidOrderLine(line.to_owned()))
            }
        };
        let file = match get_file_contents(&mut dir, line) {
            Ok(file) => file,
            Err(e) => return Err(CompileError::IOError(e, line.to_string()))
        };
        let math_file = match parse_file(file) {
            Ok(math_file) => math_file,
            Err(_) => return Err(CompileError::UnparsableFile(line.to_owned()))
        };
        match state {
            ComplierState::Waiting => (),
            ComplierState::CompilingSyntaxes => {
                let (syntax, maybe_def) = compile_syntax(math_file, &syntaxes)?;
                syntaxes.push(syntax);
                match maybe_def {
                    Some((name, def)) => {
                        let def = compile_definition(name, def, &syntaxes)?;
                        let next_ref = definitions.len() as u32;
                        references.insert(def.name.clone(), Reference::DefinitionReference(next_ref));
                        definitions.push(def);
                    },
                    None => ()
                };
            },
            ComplierState::CompilingAxioms => {
                let axiom = compile_axiom(math_file, &syntaxes)?;
                let next_ref = axioms.len() as u32;
                references.insert(axiom.name.clone(), Reference::AxiomReference(next_ref, 0));
                axioms.push(axiom);
            },
            ComplierState::CompilingTheorems => {
                let theorem = compile_theorem(
                    math_file, &syntaxes, &definitions, &axioms, &theorems, &references
                )?;
                let next_ref = theorems.len() as u32;
                references.insert(theorem.name.clone(), Reference::TheoremReference(next_ref, 0));
                theorems.push(theorem);
            }
        };
    };
    Err(CompileError::OrderFileNotFound)
}
