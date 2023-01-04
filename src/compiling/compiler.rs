use std::{fs, io, collections::HashMap};
use crate::parsing::{parse_file, MathFile};
use super::{
    Reference,
    compile_syntax, compile_definition, compile_axiom, compile_theorem,
    CompileError,
    Library
};

fn get_file_contents(dir: &mut String, filepath: &str) -> io::Result<String> {
    let initial_length = dir.len();
    dir.push_str(filepath);
    let contents = fs::read_to_string(&dir)?;
    dir.truncate(initial_length);
    Ok(contents)
}

pub fn add_syndef_to_lib(
    math_file: MathFile, lib: &mut Library, references: &mut HashMap<String, Reference>
) -> Result<(), CompileError> {
    let (syntax, maybe_def) = compile_syntax(math_file, &lib.syntaxes)?;
    lib.syntaxes.push(syntax);
    match maybe_def {
        Some((name, def)) => {
            let def = compile_definition(name, def, &lib.syntaxes)?;
            let def_ref = Reference::DefinitionReference(lib.definitions.len());
            references.insert(def.name.clone(), def_ref);
            lib.definitions.push(def);
        },
        None => ()
    };
    Ok(())
}

pub fn add_axiom_to_lib(
    math_file: MathFile, lib: &mut Library, references: &mut HashMap<String, Reference>
) -> Result<(), CompileError> {
    let axiom = compile_axiom(math_file, &lib.syntaxes)?;
    let axiom_ref = Reference::AxiomReference(lib.axioms.len(), 0);
    references.insert(axiom.name.clone(), axiom_ref);
    lib.axioms.push(axiom);
    Ok(())
}

pub fn add_theo_to_lib(
    math_file: MathFile, lib: &mut Library, references: &mut HashMap<String, Reference>
) -> Result<(), CompileError> {
    let theorem = compile_theorem(
        math_file, &lib.syntaxes, &lib.definitions, &lib.axioms, &lib.theorems, &references
    )?;
    let theo_ref = Reference::TheoremReference(lib.theorems.len(), 0);
    references.insert(theorem.name.clone(), theo_ref);
    lib.theorems.push(theorem);
    Ok(())
}

pub fn verify_theo(
    math_file: MathFile, lib: &mut Library, references: &mut HashMap<String, Reference>
) -> Result<(), CompileError> {
    let compilation_result = compile_theorem(
        math_file, &lib.syntaxes, &lib.definitions, &lib.axioms, &lib.theorems, &references
    );
    match compilation_result {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

enum CompilerState {
    CompilingSyntaxes,
    CompilingAxioms,
    CompilingTheorems,
    Waiting
}

pub fn compile_directory(mut dir: String) -> Result<Library, CompileError> {
    let mut lib = Library {
        syntaxes: Vec::new(),
        definitions: Vec::new(),
        axioms: Vec::new(),
        theorems: Vec::new()
    };
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
        let file_content = get_file_contents(&mut dir, line)
            .map_err(|e| CompileError::IOError(e, line.to_string(), line_no+1))?;
        let Ok(math_file) = parse_file(file_content) else {
            return Err(CompileError::UnparsableFile(line.to_owned(), line_no+1));
        };
        match state {
            CompilerState::Waiting => (),
            CompilerState::CompilingSyntaxes => {
                add_syndef_to_lib(math_file, &mut lib, &mut references)?;
            },
            CompilerState::CompilingAxioms => {
                add_axiom_to_lib(math_file, &mut lib, &mut references)?;
            },
            CompilerState::CompilingTheorems => {
                add_theo_to_lib(math_file, &mut lib, &mut references)?;
            }
        };
    };
    Ok(lib)
}
