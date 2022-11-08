use crate::parsing::FormulaChar;
use super::{
    Placeholder,
    Syntax, WellFormedFormula,
    CompileError
};

enum PartiallyCompiled {
    NotCompiled(FormulaChar),
    Compiled(WellFormedFormula)
}

fn are_comparable(partcomp: &PartiallyCompiled, placeholder: &Placeholder) -> bool {
    match (partcomp, placeholder) {
        (
            PartiallyCompiled::NotCompiled(FormulaChar::Char(c1)),
            Placeholder::LiteralChar(c2)
        ) => c1 == c2,
        (
            PartiallyCompiled::Compiled(WellFormedFormula::AtomicWff(_)),
            Placeholder::WellFormedFormula(_)
        ) => true,
        (
            PartiallyCompiled::Compiled(WellFormedFormula::AtomicSetvar(_)),
            Placeholder::SetVariable(_)
        ) => true,
        _ => false
    }
}

pub fn compile_formula(formula: Vec<FormulaChar>, syntaxes: &Vec<Syntax>)
-> Result<WellFormedFormula, CompileError>
{
    let mut partial_compilation = formula.into_iter()
        .map(|c|
            match c {
                FormulaChar::Char(_) => PartiallyCompiled::NotCompiled(c),
                FormulaChar::RepetitionChar => PartiallyCompiled::NotCompiled(c),
                FormulaChar::Wff(id) => PartiallyCompiled::Compiled(
                    WellFormedFormula::AtomicWff(id)
                ),
                FormulaChar::SetVar(id) => PartiallyCompiled::Compiled(
                    WellFormedFormula::AtomicSetvar(id)
                )
            }
        ).collect::<Vec<_>>();
    'try_find_patterns: while partial_compilation.len() < 1 {
        for index in 0..partial_compilation.len() {
            for (syntax_id, syntax) in syntaxes.iter().enumerate() {
                if partial_compilation.len() - index < syntax.formula.len() {
                    continue;
                };
                if partial_compilation[index..].iter()
                    .zip(&syntax.formula)
                    .all(|(c, pl)| are_comparable(c, pl)) {
                    let mut setvars = Vec::new();
                    let mut wffs = Vec::new();
                    for (c, pl) in partial_compilation[index..].iter().zip(&syntax.formula) {
                        match (c, pl) {
                            (
                                PartiallyCompiled::Compiled(f),
                                Placeholder::WellFormedFormula(_)
                            ) => wffs.push(f.clone()),
                            (
                                PartiallyCompiled::Compiled(sv),
                                Placeholder::SetVariable(_)
                            ) => setvars.push(sv.clone()),
                            _ => ()
                        }
                    };
                    for _ in 0..syntax.formula.len() {
                        partial_compilation.remove(index);
                    };
                    partial_compilation.insert(index, PartiallyCompiled::Compiled(
                        WellFormedFormula::SyntaxComposite {
                            syntax_ref: syntax_id as u32,
                            wff_parameters: wffs,
                            setvar_parameters: setvars
                        }
                    ));
                    continue 'try_find_patterns;
                }
            }
        };
        return Err(CompileError::UncompilableFormula);
    };
    match &partial_compilation[0] {
        PartiallyCompiled::Compiled(wff) => Ok(wff.clone()),
        _ => Err(CompileError::UncompilableFormula)
    }
}
