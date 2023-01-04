mod compiler;
pub use compiler::{
    add_syndef_to_lib,
    add_axiom_to_lib,
    add_theo_to_lib,
    verify_theo,
    compile_directory
};

mod types;
pub use types::{
    Syntax, SyntaxType, Placeholder,
    WellFormedFormula, Object,
    Definition, Axiom, Theorem,
    LogicStep, Reference,
    PartiallyCompiled,
    Library
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
