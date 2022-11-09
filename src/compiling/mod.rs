mod compiler;
pub use compiler::compile;

mod types;
use types::{
    Syntax, SyntaxType, Placeholder,
    WellFormedFormula, Object,
    Definition, Axiom, Theorem,
    LogicStep, Reference
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
use verification::{
    formula_is_contained,
    formula_is_substitution
};

mod error;
pub use error::CompileError;
