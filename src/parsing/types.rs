use std::collections::HashMap;

// --------------------------------------------- //
// Types for mathematical formula representation //
// --------------------------------------------- //
#[derive(Debug)]
pub enum FormulaChar {
    Char(char),
    Wff(u8),
    SetVar(u8),
    RepetitionChar
}

pub type Formula = Vec<FormulaChar>;
pub type ProofLine = (u32, Vec<u32>, String, Formula);

// --------------------------------- //
// Types for file type determination //
// --------------------------------- //
#[derive(Debug)]
pub enum FileType {
    FormulaSyntaxDefinition,
    SetVariableSyntaxDefinition,
    Axiom,
    Theorem
}

#[derive(Debug)]
pub enum FileSection {
    Syntax,
    Definition,
    HypothesisList,
    AssertionList,
    Proof,
    None
}

// ----------------------------- //
// Types for file representation //
// ----------------------------- //

#[derive(Debug)]
pub enum DefinitionType {
    Formula,
    SetVar
}

#[derive(Debug)]
pub enum MathFile {
    SyntaxDefinition {
        name: String,
        definition_type: DefinitionType,
        syntax: Formula,
        definition: Option<Formula>
    },
    Axiom {
        name: String,
        hypotheses: Vec<Formula>,
        assertions: Vec<Formula>
    },
    Theorem {
        name: String,
        hypotheses: HashMap<String, Formula>,
        assertions: Vec<Formula>,
        proof: Vec<ProofLine>
    }
}