use std::io;

#[derive(Debug)]
pub enum CompileError {
    IOError(io::Error, String),
    OrderFileNotFound,
    InvalidOrderLine(String),
    IncorrectFileType,
    UnparsableFile(String),
    AmbiguousSyntax(String),
    IncorrectNumberOfHypothesis(usize, usize, u32),
    IncorrectResultingFormula(u32),
    UnknownTheorem(String, u32),
    AssertionNotProven(u8),
    UncompilableFormula
}
