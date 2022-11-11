use super::{Placeholder, WellFormedFormula, Object};


fn equal_placeholders(p1: &Placeholder, p2: &Placeholder) -> bool {
    match (p1, p2) {
        (Placeholder::LiteralChar(c1), Placeholder::LiteralChar(c2)) => c1 == c2,
        (Placeholder::WellFormedFormula(_), Placeholder::WellFormedFormula(_)) => true,
        (Placeholder::Object(_), Placeholder::Object(_)) => true,
        (Placeholder::Repetition, Placeholder::Repetition) => true,
        _ => false
    }
}

pub fn formula_is_contained(formula1: &Vec<Placeholder>, formula2: &Vec<Placeholder>) -> bool {
    let max_len_to_check = match (formula1.len(), formula2.len()) {
        (len1, _) if len1 == 0 => return true,
        (len1, len2) if len1 > len2 => return false,
        (len1, len2) => len2 - len1 + 1
    };
    for index in 0..max_len_to_check {
        if formula2[index..].iter()
            .zip(formula1)
            .all(|(p1, p2)| equal_placeholders(p1, p2)) {
            return true;
        }
    };
    return false;
}


fn find_wff_substitutions<'a>(
    base: &WellFormedFormula, sub: &'a WellFormedFormula,
    wff_mapping: &mut Vec<Option<&'a WellFormedFormula>>, object_mapping: &mut Vec<Option<&'a Object>>
) -> Result<(), ()> {
    match (base, sub) {
        (WellFormedFormula::Atomic(id), sub) => match wff_mapping[*id] {
            None => wff_mapping[*id] = Some(sub),
            Some(wff) => {
                if sub != wff { return Err(()); }
            }
        },
        (
            WellFormedFormula::SyntaxComposite {
                syntax_ref: base_syn_ref,
                wff_parameters: base_wff_params,
                object_parameters: base_obj_params
            },
            WellFormedFormula::SyntaxComposite {
                syntax_ref: sub_syn_ref,
                wff_parameters: sub_wff_params,
                object_parameters: sub_obj_params
            }
        ) => {
            if sub_syn_ref != base_syn_ref { return Err(()); };
            for (bwp, swp) in base_wff_params.into_iter().zip(sub_wff_params) {
                match find_wff_substitutions(bwp, swp, wff_mapping, object_mapping) {
                    Ok(()) => (),
                    Err(()) => return Err(())
                }
            };
            for (bop, sop) in base_obj_params.into_iter().zip(sub_obj_params) {
                match find_object_substitutions(bop, sop, wff_mapping, object_mapping) {
                    Ok(()) => (),
                    Err(()) => return Err(())
                }
            };
        },
        _ => return Err(())
    };
    Ok(())
}

fn find_object_substitutions<'a>(
    base: &Object, sub: &'a Object,
    wff_mapping: &mut Vec<Option<&'a WellFormedFormula>>, object_mapping: &mut Vec<Option<&'a Object>>
) -> Result<(), ()> {
    match (base, sub) {
        (Object::Atomic(id), sub) => match object_mapping[*id] {
            None => object_mapping[*id] = Some(sub),
            Some(wff) => {
                if sub != wff { return Err(()); }
            }
        },
        (
            Object::SyntaxComposite {
                syntax_ref: base_syn_ref,
                wff_parameters: base_wff_params,
                object_parameters: base_obj_params
            },
            Object::SyntaxComposite {
                syntax_ref: sub_syn_ref,
                wff_parameters: sub_wff_params,
                object_parameters: sub_obj_params
            }
        ) => {
            if sub_syn_ref != base_syn_ref { return Err(()); };
            for (bwp, swp) in base_wff_params.into_iter().zip(sub_wff_params) {
                match find_wff_substitutions(bwp, swp, wff_mapping, object_mapping) {
                    Ok(()) => (),
                    Err(()) => return Err(())
                }
            };
            for (bop, sop) in base_obj_params.into_iter().zip(sub_obj_params) {
                match find_object_substitutions(bop, sop, wff_mapping, object_mapping) {
                    Ok(()) => (),
                    Err(()) => return Err(())
                }
            };
        },
        _ => return Err(())
    };
    Ok(())
}

pub fn formula_is_substitution(
    formula: &WellFormedFormula, used_hypotheses: &Vec<WellFormedFormula>,
    theo_hypotheses: &Vec<WellFormedFormula>, theo_assertion: &WellFormedFormula,
    wff_count: usize, object_count: usize
) -> bool {
    let mut wff_mapping = vec![None; wff_count];
    let mut object_mapping = vec![None; object_count];
    for (theo_hyp, used_hyp) in theo_hypotheses.into_iter().zip(used_hypotheses) {
        match find_wff_substitutions(theo_hyp, used_hyp, &mut wff_mapping, &mut object_mapping) {
            Ok(()) => (),
            Err(()) => return false
        };
    }
    match find_wff_substitutions(theo_assertion, formula, &mut wff_mapping, &mut object_mapping) {
        Ok(()) => true,
        Err(()) => false
    }
}
