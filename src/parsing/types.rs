// --------------------------------------------- //
// Types for mathematical formula representation //
// --------------------------------------------- //
#[derive(Debug)]
pub enum FormulaChar {
    Char(char),
    Wff(usize),
    Object(usize),
    RepetitionChar
}

pub type Formula = Vec<FormulaChar>;
pub type ProofLine = (usize, Vec<usize>, String, Formula);

// --------------------------------- //
// Types for file type determination //
// --------------------------------- //
#[derive(Debug)]
pub enum FileType {
    FormulaSyntaxDefinition,
    ObjectSyntaxDefinition,
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
    Object
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
        hypotheses: Vec<(String, Formula)>,
        assertions: Vec<Formula>,
        proof: Vec<ProofLine>
    }
}