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

enum CompilerState {
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
    let mut state = CompilerState::Waiting;
    let Ok(order) = get_file_contents(&mut dir, "/order.txt") else {
        return Err(CompileError::OrderFileNotFound);
    };
    for (line_no, line) in order.lines().enumerate() {
        if line == "" { continue; };
        if line.starts_with('#') {
            match (state, line) {
                (CompilerState::Waiting, "# Syntax Definitions") => {
                    state = CompilerState::CompilingSyntaxes;
                    continue;
                },
                (CompilerState::CompilingSyntaxes, "# Axioms") => {
                    state = CompilerState::CompilingAxioms;
                    continue;
                },
                (CompilerState::CompilingAxioms, "# Theorems") => {
                    state = CompilerState::CompilingTheorems;
                    continue;
                },
                _ => return Err(CompileError::InvalidOrderLine(line.to_owned(), line_no+1))
            }
        };
        let file = get_file_contents(&mut dir, line)
            .map_err(|e| CompileError::IOError(e, line.to_string(), line_no+1))?;
        let Ok(math_file) = parse_file(file) else {
            return Err(CompileError::UnparsableFile(line.to_owned(), line_no+1));
        };
        match state {
            CompilerState::Waiting => (),
            CompilerState::CompilingSyntaxes => {
                let (syntax, maybe_def) = compile_syntax(math_file, &syntaxes)?;
                syntaxes.push(syntax);
                match maybe_def {
                    Some((name, def)) => {
                        let def = compile_definition(name, def, &syntaxes)?;
                        let def_ref = Reference::DefinitionReference(definitions.len());
                        references.insert(def.name.clone(), def_ref);
                        definitions.push(def);
                    },
                    None => ()
                };
            },
            CompilerState::CompilingAxioms => {
                let axiom = compile_axiom(math_file, &syntaxes)?;
                let axiom_ref = Reference::AxiomReference(axioms.len(), 0);
                references.insert(axiom.name.clone(), axiom_ref);
                axioms.push(axiom);
            },
            CompilerState::CompilingTheorems => {
                let theorem = compile_theorem(
                    math_file, &syntaxes, &definitions, &axioms, &theorems, &references
                )?;
                let theo_ref = Reference::TheoremReference(theorems.len(), 0);
                references.insert(theorem.name.clone(), theo_ref);
                theorems.push(theorem);
            }
        };
    };
    println!("{:#?}", theorems);
    Err(CompileError::ToBeWrittenToFile)
}
