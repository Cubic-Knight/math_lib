use std::collections::HashMap;
use crate::parsing::{MathFile, FormulaChar, DefinitionType};
use super::{
    Syntax, Axiom, Theorem, Definition,
    SyntaxType, Placeholder, Reference, LogicStep,
    compile_formula,
    formula_is_contained,
    CompileError
};

pub fn compile_syntax(file: MathFile, syntaxes: &Vec<Syntax>)  // TODO: prevent syntax collisions
-> Result<(Syntax, Option<(String, Vec<FormulaChar>)>), CompileError>
{
    let (name, def_type, syntax, definition) = match file {
        MathFile::SyntaxDefinition {
            name,
            definition_type,
            syntax,
            definition
        } => (name, definition_type, syntax, definition),
        _ => return Err(CompileError::IncorrectFileType)
    };
    let syntax_type = match def_type {
        DefinitionType::Formula => SyntaxType::Formula,
        DefinitionType::SetVar => SyntaxType::SetVariable
    };
    let mut wff_mapping = [None; 32];
    let mut next_wff_id = 0;
    let mut setvar_mapping = [None; 32];
    let mut next_setvar_id = 0;
    let mut formula = Vec::new();
    for ch in syntax {
        match ch {
            FormulaChar::Char(c) => formula.push(Placeholder::LiteralChar(c)),
            FormulaChar::RepetitionChar => formula.push(Placeholder::Repetition),
            FormulaChar::Wff(id) => match wff_mapping[id as usize] {
                Some(n) => formula.push(Placeholder::WellFormedFormula(n)),
                None => {
                    formula.push(Placeholder::WellFormedFormula(next_wff_id));
                    wff_mapping[id as usize] = Some(next_wff_id);
                    next_wff_id += 1;
                }
            },
            FormulaChar::SetVar(id) => match setvar_mapping[id as usize] {
                Some(n) => formula.push(Placeholder::SetVariable(n)),
                None => {
                    formula.push(Placeholder::WellFormedFormula(next_setvar_id));
                    setvar_mapping[id as usize] = Some(next_setvar_id);
                    next_setvar_id += 1;
                }
            }
        };
    };
    // verify syntax doesn't make the compiling ambiguous
    for other_syntax in syntaxes {
        let other_formula = &other_syntax.formula;
        if formula_is_contained(&formula, other_formula) {
            return Err(CompileError::AmbiguousSyntax(name));
        };
        if formula_is_contained(other_formula, &formula) {
            return Err(CompileError::AmbiguousSyntax(name));
        }
    };

    let name_def = match definition {
        Some(def) => Some((name, def)),
        None => None
    };
    Ok((
        Syntax {
            syntax_type,
            formula,
            distinct_wff_count: next_wff_id,
            distinct_setvar_count: next_setvar_id
        },
        name_def
    ))
}

pub fn compile_definition(name: String, def: Vec<FormulaChar>, syntaxes: &Vec<Syntax>) -> Result<Definition, CompileError> {
    Ok(
        Definition {
            name,
            definition: compile_formula(def, syntaxes)?
        }
    )
}

pub fn compile_axiom(file: MathFile, syntaxes: &Vec<Syntax>) -> Result<Axiom, CompileError> {
    let (name, hypotheses, assertions) = match file {
        MathFile::Axiom {
            name,
            hypotheses,
            assertions
        } => (name, hypotheses, assertions),
        _ => return Err(CompileError::IncorrectFileType)
    };
    let compiled_hypotheses = hypotheses.into_iter()
        .map(|hyp| compile_formula(hyp, syntaxes))
        .collect::<Result<Vec<_>, _>>()?;
    let compiled_assertions = assertions.into_iter()
        .map(|ass| compile_formula(ass, syntaxes))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Axiom {
        name,
        hypotheses: compiled_hypotheses,
        assertions: compiled_assertions
    })
}

pub fn compile_theorem(
    file: MathFile,
    syntaxes: &Vec<Syntax>,
    definitions: &Vec<Definition>,
    axioms: &Vec<Axiom>,
    theorems: &Vec<Theorem>,
    references: &HashMap<String, Reference> 
) -> Result<Theorem, CompileError>
{
    let (name, hypotheses, assertions, proof) = match file {
        MathFile::Theorem {
            name,
            hypotheses,
            assertions,
            proof
        } => (name, hypotheses, assertions, proof),
        _ => return Err(CompileError::IncorrectFileType)
    };
    let mut hypot_names = HashMap::new();
    let mut hypot_list = Vec::new();
    for (index, (hypot_name, hypot)) in hypotheses.into_iter().enumerate() {
        hypot_names.insert(hypot_name, index);
        hypot_list.push(compile_formula(hypot, syntaxes)?);
    }
    let assertions = assertions.into_iter()
        .map(|ass| compile_formula(ass, syntaxes))
        .collect::<Result<Vec<_>, _>>()?;
    // Proof compilation and verification
    let mut compilied_proof = Vec::new();
    for (index, used_hypots, theorem_name, formula) in proof {
        let resulting_formula = compile_formula(formula, syntaxes)?;
        // Case of hypothesis usage
        match hypot_names.get(&theorem_name) {
            None => (),
            Some(&hypot_id) => {
                if used_hypots.len() != 0 {
                    return Err(CompileError::IncorrectNumberOfHypothesis(used_hypots.len(), 0, index));
                };
                if resulting_formula != hypot_list[hypot_id] {
                    return Err(CompileError::IncorrectResultingFormula(index));
                };
                compilied_proof.push(LogicStep {
                    used_hypotheses: used_hypots,  // Empty
                    theorem_ref: Reference::HypothesisReference(hypot_id as u8),
                    resulting_formula
                });
                continue;
            }
        };
        // Case of definition/axiom/theorem usage
        let (theo_name, assert_id) = match theorem_name.split_once('.') {
            None => (theorem_name, 0),
            Some((name, id)) => {
                match id.parse::<u8>() {
                    Ok(n) => (name.to_owned(), n),
                    Err(_) => return Err(CompileError::UnknownTheorem(theorem_name, index))
                }
            }
        };
        match references.get(&theo_name) {
            Some(Reference::DefinitionReference(def_id)) => {
                let def = &definitions[*def_id as usize];
                if used_hypots.len() != 0 {
                    return Err(CompileError::IncorrectNumberOfHypothesis(used_hypots.len(), 0, index));
                };
                if resulting_formula != def.definition {
                    return Err(CompileError::IncorrectResultingFormula(index));
                };
                compilied_proof.push(LogicStep {
                    used_hypotheses: used_hypots,  // Empty
                    theorem_ref: Reference::DefinitionReference(*def_id),
                    resulting_formula
                });
            },
            Some(Reference::AxiomReference(ax_id, 0)) => {
                let ax = &axioms[*ax_id as usize];
                if used_hypots.len() != ax.hypotheses.len() {
                    return Err(CompileError::IncorrectNumberOfHypothesis(
                        used_hypots.len(), ax.hypotheses.len(), index
                    ));
                };
                if resulting_formula != ax.assertions[assert_id as usize] {
                    return Err(CompileError::IncorrectResultingFormula(index));
                };
                compilied_proof.push(LogicStep {
                    used_hypotheses: used_hypots,
                    theorem_ref: Reference::AxiomReference(*ax_id, assert_id),
                    resulting_formula
                })
            },
            Some(Reference::TheoremReference(theo_id, 0)) => {
                let theo = &theorems[*theo_id as usize];
                if used_hypots.len() != theo.assertions.len() {
                    return Err(CompileError::IncorrectNumberOfHypothesis(
                        used_hypots.len(), theo.hypotheses.len(), index
                    ));
                };
                if resulting_formula != theo.assertions[assert_id as usize] {
                    return Err(CompileError::IncorrectResultingFormula(index));
                };
                compilied_proof.push(LogicStep {
                    used_hypotheses: used_hypots,
                    theorem_ref: Reference::TheoremReference(*theo_id, assert_id),
                    resulting_formula
                })
            },
            _ => return Err(CompileError::UnknownTheorem(theo_name, index))
        };
    };
    let steps = compilied_proof.iter()
        .map(|l| &l.resulting_formula)
        .collect::<Vec<_>>();
    for (index, assertion) in assertions.iter().enumerate() {
        if !steps.contains(&assertion) {
            return Err(CompileError::AssertionNotProven(index as u8));
        };
    };

    Ok(Theorem {
        name,
        hypotheses: hypot_list,
        assertions,
        proof: compilied_proof
    })
}
