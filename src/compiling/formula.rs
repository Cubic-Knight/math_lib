use std::collections::HashMap;
use crate::parsing::FormulaChar;
use super::{
    PartiallyCompiled,
    Placeholder,
    Syntax, SyntaxType,
    WellFormedFormula, Object,
    CompileError
};

pub fn compile_formula(
    formula: Vec<FormulaChar>, syntaxes: &Vec<Syntax>,
    wffs: &mut HashMap<usize, WellFormedFormula>, objects: &mut HashMap<usize, Object>
) -> Result<WellFormedFormula, CompileError> {
    let mut next_wff_index = wffs.len();
    let mut next_object_index = objects.len();
    let mut partial_compilation = formula.into_iter()
        .map(|c| match c {
            FormulaChar::Char(_) => Ok(PartiallyCompiled::NotCompiled(c)),
            FormulaChar::RepetitionChar => Err(CompileError::RepetitionCharacterNotCompilable),
            FormulaChar::Wff(id) => match wffs.get(&id) {
                Some(wff) => Ok(PartiallyCompiled::CompiledFormula(wff.to_owned())),
                None => {
                    let wff = WellFormedFormula::Atomic(next_wff_index);
                    wffs.insert(id, wff.clone());
                    next_wff_index += 1;
                    Ok(PartiallyCompiled::CompiledFormula(wff))
                }
            },
            FormulaChar::Object(id) => match objects.get(&id) {
                Some(object) => Ok(PartiallyCompiled::CompiledObject(object.to_owned())),
                None => {
                    let object = Object::Atomic(next_object_index);
                    objects.insert(id, object.clone());
                    next_object_index += 1;
                    Ok(PartiallyCompiled::CompiledObject(object))
                }
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    'try_find_patterns: while partial_compilation.len() > 1 {
        'try_syntax: for (syntax_id, syntax) in syntaxes.iter().enumerate() {
            'try_position: for index in 0..partial_compilation.len() {
                if partial_compilation.len() - index < syntax.formula.len() {
                    continue 'try_syntax;  // If the syntax does not fit in the rest of the text, go to the next syntax
                };
                let mut wffs = vec![None; syntax.distinct_wff_count];
                let mut objects = vec![None; syntax.distinct_object_count];
                for (c, pl) in partial_compilation[index..].iter().zip(&syntax.formula) {
                    let valid = match (c, pl) {
                        (
                            PartiallyCompiled::NotCompiled(FormulaChar::Char(c1)),
                            Placeholder::LiteralChar(c2)
                        ) => c1 == c2,
                        (
                            PartiallyCompiled::CompiledFormula(wff),
                            Placeholder::WellFormedFormula(id)
                        ) => match &wffs[*id] {
                            Some(f) => wff == f,
                            None => { wffs[*id] = Some(wff.to_owned()); true }
                        },
                        (
                            PartiallyCompiled::CompiledObject(obj),
                            Placeholder::Object(id)
                        ) => match &objects[*id] {
                            Some(o) => obj == o,
                            None => { objects[*id] = Some(obj.to_owned()); true }
                        },
                        _ => false
                    };
                    match valid {
                        true => (),
                        false => continue 'try_position  // Syntax did not match here, try next position
                    };
                };
                let wff_parameters = wffs.into_iter()
                    .map(|wff| wff.ok_or(CompileError::ShouldNotBeReached))
                    .collect::<Result<Vec<_>, _>>()?;
                let object_parameters = objects.into_iter()
                    .map(|obj| obj.ok_or(CompileError::ShouldNotBeReached))
                    .collect::<Result<Vec<_>, _>>()?;
                let element_to_insert = match syntax.syntax_type {
                    SyntaxType::Formula => PartiallyCompiled::CompiledFormula(
                        WellFormedFormula::SyntaxComposite { syntax_ref: syntax_id, wff_parameters, object_parameters }
                    ),
                    SyntaxType::Object => PartiallyCompiled::CompiledObject(
                        Object::SyntaxComposite { syntax_ref: syntax_id, wff_parameters, object_parameters }
                    )
                };
                for _ in 0..syntax.formula.len() {
                    partial_compilation.remove(index);
                };
                partial_compilation.insert(index, element_to_insert);
                // Once a syntax has been compiled, we iterate through the list again from the beginning
                continue 'try_find_patterns;
            };
        };
        // We only can get here if no syntax has matched
        return Err(CompileError::UncompilableFormula(partial_compilation));
    };
    match partial_compilation.pop() {
        Some(PartiallyCompiled::CompiledFormula(wff)) => Ok(wff),
        Some(other) => Err(CompileError::UncompilableFormula(vec![other])),
        None => Err(CompileError::ShouldNotBeReached)
    }
}
