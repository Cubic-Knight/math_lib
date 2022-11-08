use std::collections::HashMap;


use super::{
    Formula, FormulaChar,  // Formula is an alias for 'Vec<FormulaChar>'
    ProofLine  // ProofLine is an alias for '(u32, Vec<u32>, String, Formula)'
};

pub fn parse_formula(fm: &str) -> Formula {
    let mut res = Formula::new();
    for c in fm.chars() {
        if c == ' ' { continue; }
        if c == '…' {
            res.push(FormulaChar::RepetitionChar);
        } else if '𝑎' <= c && c <= '𝑧' {  // '𝑎' and '𝑧' here are NOT ascii
            let id = c as u32 - '𝑎' as u32;
            res.push(FormulaChar::SetVar(id as u8));
        } else if '𝛼' <= c && c <= '𝜔' {
            let id = c as u32 - '𝛼' as u32;
            res.push(FormulaChar::Wff(id as u8))
        } else {
            res.push(FormulaChar::Char(c))
        };
    };
    res
}

pub fn parse_named_formula(nfm: &str) -> Result<(String, Formula), ()> {
    let (name, formula) = match nfm.split_once(':') {
        Some((name, formula)) => (name, formula),
        None => return Err(())
    };
    let name = name.trim().to_owned();
    let formula = parse_formula(formula);
    Ok((name, formula))
}

pub fn parse_proof_line(prline: &str) -> Result<ProofLine, ()> {
    let mut split = prline.splitn(4, ';');

    let line_no: u32 = match split.next().map(|s| s.trim().parse()) {
        Some(Ok(line_no)) => line_no,
        _ => return Err(())
    };
    let used_hypots: Vec<u32> = match split.next() {
        Some(used_hypots) => {
            let splitted = used_hypots.split(',')
                .filter(|s| s.trim().len() > 0)
                .map(|s| s.trim().parse())
                .collect();
            match splitted {
                Ok(vec) => vec,
                Err(_) => return Err(())
            }
        },
        None => return Err(())
    };
    let theorem_reference = match split.next() {
        Some(theorem_reference) => theorem_reference.trim().to_owned(),
        None => return Err(())
    };
    let formula = match split.next() {
        Some(formula) => parse_formula(formula),
        None => return Err(())
    };
    Ok((line_no, used_hypots, theorem_reference, formula))
}
