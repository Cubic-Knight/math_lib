mod types;
use types::FileType;
pub use types::{
    FileLine, ProvenState,
    ColoredString, Color
};

mod parser;
pub use parser::parse_file;

mod sections;
use sections::{
    parse_syntax_section,
    parse_definition_section,
    parse_hypotesis_section,
    parse_assertion_section,
    parse_proof_section
};

mod formula;
use formula::{
    parse_new_syntax,
    parse_formula
};
