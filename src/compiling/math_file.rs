use std::collections::HashMap;
use crate::parsing::{MathFile, FormulaChar, DefinitionType};
use super::{
    Syntax, Axiom, Theorem, Definition,
    SyntaxType, Placeholder, Reference, LogicStep,
    WellFormedFormula, Object,
    compile_formula,
    formula_is_contained,
    formula_is_substitution,
    CompileError
};

pub fn compile_syntax(file: MathFile, syntaxes: &Vec<Syntax>)
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
        DefinitionType::Object => SyntaxType::Object
    };
    let mut wff_mapping = [None; 32];
    let mut next_wff_id = 0;
    let mut obj_mapping = [None; 32];
    let mut next_obj_id = 0;
    let mut formula = Vec::new();
    for ch in syntax {
        match ch {
            FormulaChar::Char(c) => formula.push(Placeholder::LiteralChar(c)),
            FormulaChar::RepetitionChar => formula.push(Placeholder::Repetition),
            FormulaChar::Wff(id) => match wff_mapping[id] {
                Some(n) => formula.push(Placeholder::WellFormedFormula(n)),
                None => {
                    formula.push(Placeholder::WellFormedFormula(next_wff_id));
                    wff_mapping[id] = Some(next_wff_id);
                    next_wff_id += 1;
                }
            },
            FormulaChar::Object(id) => match obj_mapping[id] {
                Some(n) => formula.push(Placeholder::Object(n)),
                None => {
                    formula.push(Placeholder::WellFormedFormula(next_obj_id));
                    obj_mapping[id] = Some(next_obj_id);
                    next_obj_id += 1;
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
            distinct_object_count: next_obj_id
        },
        name_def
    ))
}

pub fn compile_definition(name: String, def: Vec<FormulaChar>, syntaxes: &Vec<Syntax>) -> Result<Definition, CompileError> {
    let mut wffs = HashMap::<usize, WellFormedFormula>::new();
    let mut objects = HashMap::<usize, Object>::new();
    let definition = compile_formula(def, syntaxes, &mut wffs, &mut objects)?;
    Ok(
        Definition {
            name,
            definition,
            distinct_wff_count: wffs.len(),
            distinct_object_count: objects.len()
        }
    )
}

pub fn compile_axiom(file: MathFile, syntaxes: &Vec<Syntax>) -> Result<Axiom, CompileError> {
    let mut wffs = HashMap::<usize, WellFormedFormula>::new();
    let mut objects = HashMap::<usize, Object>::new();
    let (name, hypotheses, assertions) = match file {
        MathFile::Axiom {
            name,
            hypotheses,
            assertions
        } => (name, hypotheses, assertions),
        _ => return Err(CompileError::IncorrectFileType)
    };
    let compiled_hypotheses = hypotheses.into_iter()
        .map(|hyp| compile_formula(hyp, syntaxes, &mut wffs, &mut objects))
        .collect::<Result<Vec<_>, _>>()?;
    let compiled_assertions = assertions.into_iter()
        .map(|ass| compile_formula(ass, syntaxes, &mut wffs, &mut objects))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Axiom {
        name,
        hypotheses: compiled_hypotheses,
        assertions: compiled_assertions,
        distinct_wff_count: wffs.len(),
        distinct_object_count: objects.len()
    })
}

pub fn compile_theorem(
    file: MathFile,
    syntaxes: &Vec<Syntax>,
    definitions: &Vec<Definition>,
    axioms: &Vec<Axiom>,
    theorems: &Vec<Theorem>,
    references: &HashMap<String, Reference> 
) -> Result<Theorem, CompileError> {
    let mut wffs = HashMap::<usize, WellFormedFormula>::new();
    let mut objects = HashMap::<usize, Object>::new();
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
        hypot_list.push(compile_formula(hypot, syntaxes, &mut wffs, &mut objects)?);
    }
    let assertions = assertions.into_iter()
        .map(|ass| compile_formula(ass, syntaxes, &mut wffs, &mut objects))
        .collect::<Result<Vec<_>, _>>()?;
    // Proof compilation and verification
    let mut compiled_proof = Vec::new();
    for (
        i,
        (index, used_hypots, theorem_name, formula)
    ) in proof.into_iter().enumerate() {
        if index != i + 1 { return Err(CompileError::MissingProofLine(i+1)); };
        // Hypothesis usage
        if let Some(&hypot_id) = hypot_names.get(&theorem_name) {
            if used_hypots.len() != 0 {
                return Err(CompileError::IncorrectNumberOfHypothesis(used_hypots.len(), 0, index));
            };
            let resulting_formula = compile_formula(formula, syntaxes, &mut wffs, &mut objects)?;
            if resulting_formula != hypot_list[hypot_id] {
                return Err(CompileError::IncorrectResultingFormula(index));
            };
            compiled_proof.push(LogicStep {
                used_hypotheses: used_hypots,  // Empty
                theorem_ref: Reference::HypothesisReference(hypot_id),
                resulting_formula
            });
            continue;
        };
        // Definition/Axiom/Theorem usage
        let (theo_name, assert_id) = match theorem_name.split_once('.') {
            None => (theorem_name, 0),
            Some((name, id)) => {
                let Ok(id) = id.parse::<usize>() else {
                    return Err(CompileError::UnknownTheorem(theorem_name, index));
                };
                (name.to_owned(), id)
            }
        };
        let Some(reference) = references.get(&theo_name) else {
            return Err(CompileError::UnknownTheorem(theo_name, index));
        };
        let empty_vec = vec![];
        let (theo_hypotheses, theo_assertion,
            wff_count, object_count, theo_ref) = match reference {
            Reference::DefinitionReference(def_id) => {
                match &definitions[*def_id] {
                    Definition {
                        name: _,
                        definition,
                        distinct_wff_count,
                        distinct_object_count
                    } => {
                        let def_ref = Reference::DefinitionReference(*def_id);
                        (&empty_vec, definition.clone(), *distinct_wff_count, *distinct_object_count, def_ref)
                    }
                }
            },
            Reference::AxiomReference(ax_id, 0) => {
                match &axioms[*ax_id] {
                    Axiom {
                        name: _,
                        hypotheses,
                        assertions,
                        distinct_wff_count,
                        distinct_object_count
                    } => {
                        let Some(assertion) = assertions.get(assert_id) else {
                            return Err(CompileError::UnknownTheorem(theo_name + "." + &assert_id.to_string(), index));
                        };
                        let ax_ref = Reference::AxiomReference(*ax_id, assert_id);
                        (hypotheses, assertion.clone(), *distinct_wff_count, *distinct_object_count, ax_ref)
                    }
                }
            },
            Reference::TheoremReference(theo_id, 0) => {
                match &theorems[*theo_id] {
                    Theorem {
                        name: _,
                        hypotheses,
                        assertions,
                        proof: _,
                        distinct_wff_count,
                        distinct_object_count
                    } => {
                        let Some(assertion) = assertions.get(assert_id) else {
                            return Err(CompileError::UnknownTheorem(theo_name + "." + &assert_id.to_string(), index));
                        };
                        let theo_ref = Reference::TheoremReference(*theo_id, assert_id);
                        (hypotheses, assertion.clone(), *distinct_wff_count, *distinct_object_count, theo_ref)
                    }
                }
            },
            _ => return Err(CompileError::WeirdReference)
        };
        if used_hypots.len() != theo_hypotheses.len() {
            return Err(CompileError::IncorrectNumberOfHypothesis(used_hypots.len(), theo_hypotheses.len(), index));
        };
        let used_hypots = used_hypots.into_iter()
            .map(|n| n.checked_sub(1).ok_or(CompileError::InaccessibleHypothesis(0, index)))
            .collect::<Result<Vec<_>, _>>()?;
        let used_hypotheses = used_hypots.iter()
            .map(|idx| compiled_proof.get(*idx)
                .ok_or(CompileError::InaccessibleHypothesis(idx+1, index))
                .map(|step| step.resulting_formula.clone())
            ).collect::<Result<Vec<_>, _>>()?;
        let resulting_formula = compile_formula(formula, syntaxes, &mut wffs, &mut objects)?;
        if !formula_is_substitution(&resulting_formula, &used_hypotheses, &theo_hypotheses, &theo_assertion, wff_count, object_count) {
            return Err(CompileError::IncorrectResultingFormula(index));
        };
        compiled_proof.push(LogicStep {
            used_hypotheses: used_hypots,
            theorem_ref: theo_ref,
            resulting_formula
        });
    };
    // Verify that assertions have been proven
    let steps = compiled_proof.iter()
        .map(|l| &l.resulting_formula)
        .collect::<Vec<_>>();
    for (index, assertion) in assertions.iter().enumerate() {
        if !steps.contains(&assertion) {
            return Err(CompileError::AssertionNotProven(index));
        };
    };

    Ok(Theorem {
        name,
        hypotheses: hypot_list,
        assertions,
        proof: compiled_proof,
        distinct_wff_count: wffs.len(),
        distinct_object_count: objects.len()
    })
}
