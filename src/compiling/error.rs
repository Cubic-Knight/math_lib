use std::io;

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
    AssertionNotProven(usize),
    InaccessibleHypothesis(usize),

    // TODO error
    ToBeWrittenToFile
}
