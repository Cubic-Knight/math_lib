use std::{
    fs::File, 
    io::{self, Write, BufReader, Read}};
use crate::compiling::{
    Syntax, Definition,
    Axiom, Theorem,
    Library
};
use super::BinaryConvert;

pub fn write_lib(path: String, lib: Library) -> io::Result<()> {
    let Library {
        syntaxes,
        definitions,
        axioms,
        theorems
    } = lib;
    let mut file = File::create(path)?;
    let mut data = Vec::new();
    for syntax in syntaxes {
        data.push(0xf0);
        data.append(&mut syntax.to_binary());
    };
    for definition in definitions {
        data.push(0xf1);
        data.append(&mut definition.to_binary());
    };
    for axiom in axioms {
        data.push(0xf2);
        data.append(&mut axiom.to_binary());
    };
    for theorem in theorems {
        data.push(0xf3);
        data.append(&mut theorem.to_binary());
    };
    data.push(0xf4);  // EOF
    file.write_all(&data)?;
    Ok(())
}

pub fn read_file(path: String) -> io::Result<Library> {
    let buf = BufReader::new(File::open(path)?);
    let mut source = buf.bytes()
        .take_while(|item| item.is_ok())
        .filter_map(|item| item.ok());
    let mut lib = Library {
        syntaxes: Vec::new(),
        definitions: Vec::new(),
        axioms: Vec::new(),
        theorems: Vec::new()
    };
    loop {
        match source.next() {
            Some(0xf0) => {
                let Some(syntax) = Syntax::from_binary(&mut source) else {
                    continue;
                };
                lib.syntaxes.push(syntax)
            },
            Some(0xf1) => {
                let Some(definition) = Definition::from_binary_syntaxes(&mut source, &lib.syntaxes) else {
                    continue;
                };
                lib.definitions.push(definition)
            },
            Some(0xf2) => {
                let Some(axiom) = Axiom::from_binary_syntaxes(&mut source, &lib.syntaxes) else {
                    continue;
                };
                lib.axioms.push(axiom)
            },
            Some(0xf3) => {
                let Some(theorem) = Theorem::from_binary_syntaxes(&mut source, &lib.syntaxes) else {
                    continue;
                };
                lib.theorems.push(theorem)
            },
            Some(0xf4) => {
                break;
            },
            Some(_) => continue,
            None => break
        };
    };
    Ok(lib)
}
