use std::{
    io::{self, BufReader, Read},
    fs::File,
    collections::HashMap
};
use super::{
    Syntax, Definition, Axiom, Theorem,
    Reference, LibraryData,

    FromBinary
};

pub fn read_lib_data() -> io::Result<(LibraryData, HashMap<String, Reference>)> {
    let buf = BufReader::new(File::open("library.math")?);
    let mut source = buf.bytes()
        .take_while(|item| item.is_ok())
        .filter_map(|item| item.ok());
    let mut lib = LibraryData {
        syntaxes: Vec::new(),
        definitions: Vec::new(),
        axioms: Vec::new(),
        theorems: Vec::new()
    };
    let mut references = HashMap::new();
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
                let def_ref = Reference::DefinitionReference(lib.definitions.len());
                references.insert(definition.name.clone(), def_ref);
                lib.definitions.push(definition)
            },
            Some(0xf2) => {
                let Some(axiom) = Axiom::from_binary_syntaxes(&mut source, &lib.syntaxes) else {
                    continue;
                };
                let ax_ref = Reference::AxiomReference(lib.axioms.len(), 0);
                references.insert(axiom.name.clone(), ax_ref);
                lib.axioms.push(axiom)
            },
            Some(0xf3) => {
                let Some(theorem) = Theorem::from_binary_syntaxes(&mut source, &lib.syntaxes) else {
                    continue;
                };
                let theo_ref = Reference::TheoremReference(lib.theorems.len(), 0);
                references.insert(theorem.name.clone(), theo_ref);
                lib.theorems.push(theorem)
            },
            Some(0xf4) => {
                break;
            },
            Some(_) => continue,
            None => break
        };
    };
    Ok((lib, references))
}
