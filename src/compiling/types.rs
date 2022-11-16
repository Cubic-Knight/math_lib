#[derive(Debug)]
pub enum Placeholder {
    LiteralChar(char),
    WellFormedFormula(usize),
    Object(usize),
    Repetition
}

#[derive(Debug)]
pub enum SyntaxType {
    Formula,
    Object
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Definition {
    pub name: String,
    pub definition: WellFormedFormula,
    pub distinct_wff_count: usize,
    pub distinct_object_count: usize
}

#[derive(Debug)]
pub struct Axiom {
    pub name: String,
    pub hypotheses: Vec<WellFormedFormula>,
    pub assertions: Vec<WellFormedFormula>,
    pub distinct_wff_count: usize,
    pub distinct_object_count: usize
}

#[derive(Debug)]
pub enum Reference {
    HypothesisReference(usize),
    DefinitionReference(usize),
    AxiomReference(usize, usize),
    TheoremReference(usize, usize)
}

#[derive(Debug)]
pub struct LogicStep {
    pub used_hypotheses: Vec<usize>,
    pub theorem_ref: Reference,
    pub resulting_formula: WellFormedFormula
}

#[derive(Debug)]
pub struct Theorem {
    pub name: String,
    pub hypotheses: Vec<WellFormedFormula>,
    pub assertions: Vec<WellFormedFormula>,
    pub proof: Vec<LogicStep>,
    pub distinct_wff_count: usize,
    pub distinct_object_count: usize
}

use crate::parsing::FormulaChar;
#[derive(Debug)]
pub enum PartiallyCompiled {
    NotCompiled(FormulaChar),
    CompiledFormula(WellFormedFormula),
    CompiledObject(Object)
}

#[derive(Debug)]
pub struct Library {
    pub syntaxes: Vec<Syntax>,
    pub definitions: Vec<Definition>,
    pub axioms: Vec<Axiom>,
    pub theorems: Vec<Theorem>
}
