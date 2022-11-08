pub enum Placeholder {
    LiteralChar(char),
    WellFormedFormula(u8),
    SetVariable(u8),
    Repetition
}

pub enum SyntaxType {
    Formula,
    SetVariable
}

pub struct Syntax {
    pub syntax_type: SyntaxType,
    pub formula: Vec<Placeholder>,
    pub distinct_wff_count: u8,
    pub distinct_setvar_count: u8
}

#[derive(PartialEq, Clone)]
pub enum WellFormedFormula {
    AtomicWff(u8),
    AtomicSetvar(u8),
    SyntaxComposite {
        syntax_ref: u32,
        wff_parameters: Vec<WellFormedFormula>,
        setvar_parameters: Vec<WellFormedFormula>
    }
}

pub struct Definition {
    pub name: String,
    pub definition: WellFormedFormula
}

pub struct Axiom {
    pub name: String,
    pub hypotheses: Vec<WellFormedFormula>,
    pub assertions: Vec<WellFormedFormula>
}

pub enum Reference {
    HypothesisReference(u8),
    DefinitionReference(u32),
    AxiomReference(u32, u8),
    TheoremReference(u32, u8)
}

pub struct LogicStep {
    pub used_hypotheses: Vec<u32>,
    pub theorem_ref: Reference,
    pub resulting_formula: WellFormedFormula
}

pub struct Theorem {
    pub name: String,
    pub hypotheses: Vec<WellFormedFormula>,
    pub assertions: Vec<WellFormedFormula>,
    pub proof: Vec<LogicStep>
}
