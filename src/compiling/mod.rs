mod compiler;
pub use compiler::compile;

mod types;
use types::{
    Placeholder, Syntax, SyntaxType,
    WellFormedFormula, Definition, Axiom,
    LogicStep, Reference, Theorem
};

mod math_file;
use math_file::{
    compile_syntax,
    compile_definition,
    compile_axiom,
    compile_theorem
};

mod formula;
use formula::compile_formula;

mod verification;
use verification::{formula_is_contained};

mod error;
pub use error::CompileError;
