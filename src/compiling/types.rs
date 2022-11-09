pub enum Placeholder {
    LiteralChar(char),
    WellFormedFormula(usize),
    Object(usize),
    Repetition
}

pub enum SyntaxType {
    Formula,
    Object
}

pub struct Syntax {
    pub syntax_type: SyntaxType,
    pub formula: Vec<Placeholder>,
    pub distinct_wff_count: usize,
    pub distinct_object_count: usize
}

#[derive(Debug, PartialEq, Clone)]
pub enum WellFormedFormula {
    Atomic(usize),
    SyntaxComposite {
        syntax_ref: usize,
        wff_parameters: Vec<WellFormedFormula>,
        object_parameters: Vec<Object>
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Atomic(usize),
    SyntaxComposite {
        syntax_ref: usize,
        wff_parameters: Vec<WellFormedFormula>,
        object_parameters: Vec<Object>
    }
}

pub struct Definition {
    pub name: String,
    pub definition: WellFormedFormula,
    pub distinct_wff_count: usize,
    pub distinct_object_count: usize
}

pub struct Axiom {
    pub name: String,
    pub hypotheses: Vec<WellFormedFormula>,
    pub assertions: Vec<WellFormedFormula>,
    pub distinct_wff_count: usize,
    pub distinct_object_count: usize
}

pub enum Reference {
    HypothesisReference(usize),
    DefinitionReference(usize),
    AxiomReference(usize, usize),
    TheoremReference(usize, usize)
}

pub struct LogicStep {
    pub used_hypotheses: Vec<usize>,
    pub theorem_ref: Reference,
    pub resulting_formula: WellFormedFormula
}

pub struct Theorem {
    pub name: String,
    pub hypotheses: Vec<WellFormedFormula>,
    pub assertions: Vec<WellFormedFormula>,
    pub proof: Vec<LogicStep>,
    pub distinct_wff_count: usize,
    pub distinct_object_count: usize
}
