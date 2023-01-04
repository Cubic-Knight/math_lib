pub enum FileType {
    SyntaxDefinitionFormula,
    SyntaxDefinitionObject,
    Axiom,
    Theorem,
    Unknown
}

pub enum FileLine {
    Raw(String),
    Title { title: String, name: String, title_good: bool, name_good: bool },
    Section { name: String, is_valid: bool },
    Hypothesis { name: String, hypot: ColoredString },
    Assertion { assertion: ColoredString, is_proven: ProvenState },
    ProofLine {
        line_no: String, line_index: usize,
        used_hypots: Vec<String>,
        theo_ref: String, theo_ref_exists: bool,
        formula: ColoredString
    },
    UnexpectedLine(String)
}

pub enum ProvenState {
    NotProven,
    Proven,
    Assumed,
    None
}

pub struct ColoredString {
    pub characters: Vec<char>,
    pub colors: Vec<Color>
}

#[derive(Debug)]
#[derive(PartialEq, Clone, Copy)]
pub enum Color {
    Normal = 0,
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37
}
