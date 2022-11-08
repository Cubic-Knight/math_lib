use super::Placeholder;


fn equal_placeholders(p1: &Placeholder, p2: &Placeholder) -> bool {
    match (p1, p2) {
        (Placeholder::LiteralChar(c1), Placeholder::LiteralChar(c2)) => c1 == c2,
        (Placeholder::WellFormedFormula(_), Placeholder::WellFormedFormula(_)) => true,
        (Placeholder::SetVariable(_), Placeholder::SetVariable(_)) => true,
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
