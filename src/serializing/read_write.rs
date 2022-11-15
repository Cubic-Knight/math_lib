use std::{io::{self, Write}, fs::File};
use crate::compiling::{
    Syntax, Definition,
    Axiom, Theorem
};
use super::BinaryConvert;

pub fn write_lib(
    path: String,
    syntaxes: Vec<Syntax>, definitions: Vec<Definition>,
    axioms: Vec<Axiom>, theorems: Vec<Theorem>
) -> io::Result<()> {
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
    file.write_all(&data)?;
    Ok(())
}
