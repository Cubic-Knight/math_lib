use std::io;
use super::PartiallyCompiled;

#[derive(Debug)]
pub enum CompileError {
    // In compiler.rs
    OrderFileNotFound,
    InvalidOrderLine(String, usize),
    IOError(io::Error, String, usize),
    UnparsableFile(String, usize),

    // In math_file.rs
    IncorrectFileType,
    AmbiguousSyntax(String),
    MissingProofLine(usize),
    IncorrectNumberOfHypothesis(usize, usize, usize),
    IncorrectResultingFormula(usize),
    UnknownTheorem(String, usize),
    WeirdReference,
    InaccessibleHypothesis(usize, usize),
    AssertionNotProven(usize),

    // In formula.rs
    ShouldNotBeReached,
    UncompilableFormula(Vec<PartiallyCompiled>),

    // TODO error
    RepetitionCharacterNotCompilable,
}
