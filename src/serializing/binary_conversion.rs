use super::{
    BinaryConvert, Vectorizable,
    RpnBlock,
    wff_to_rpn, rpn_to_wff
};
use crate::compiling::{
    Syntax, SyntaxType, Placeholder,
    Definition, Axiom,
    Theorem, LogicStep, Reference
};

impl BinaryConvert<0> for u8 {
    fn to_binary(self) -> Vec<u8> {
        vec![self]
    }
    fn from_binary<I: Iterator<Item = u8>>(source: &mut I) -> Option<Self> {
        source.next()
    }
    fn from_binary_syntaxes<I: Iterator<Item = u8>>(source: &mut I, _syntaxes: &Vec<Syntax>) -> Option<Self> {
        Self::from_binary(source)
    }
}

impl BinaryConvert<0> for usize {
    fn to_binary(self) -> Vec<u8> {
        (self as u32).to_le_bytes().to_vec()
    }
    fn from_binary<I: Iterator<Item = u8>>(source: &mut I) -> Option<Self> {
        let mut bytes = [0; 4];
        for (i, data) in (0..4).map(|_| source.next()).enumerate() {
            bytes[i] = data?;
        };
        Some(u32::from_le_bytes(bytes) as usize)
    }
    fn from_binary_syntaxes<I: Iterator<Item = u8>>(source: &mut I, _syntaxes: &Vec<Syntax>) -> Option<Self> {
        Self::from_binary(source)
    }
}

impl BinaryConvert<0> for String {
    fn to_binary(self) -> Vec<u8> {
        let mut res = self.into_bytes();
        res.push(0x00);  // String terminator
        res
    }
    fn from_binary<I: Iterator<Item = u8>>(source: &mut I) -> Option<Self> {
        let string = source.take_while(|n| n != &0x00)
            .collect::<Vec<_>>();
        String::from_utf8(string).ok()
    }
    fn from_binary_syntaxes<I: Iterator<Item = u8>>(source: &mut I, _syntaxes: &Vec<Syntax>) -> Option<Self> {
        Self::from_binary(source)
    }
}

impl<T, const N: usize> BinaryConvert<N> for Vec<T>
where T: Vectorizable<BinaryForm = [u8; N]> {
    fn to_binary(self) -> Vec<u8> {
        let mut res =  Vec::new();
        for element in self {
            res.extend_from_slice(&element.to_binary_in_vec());
        };
        res.extend_from_slice(&T::TERMINATOR.clone());
        res
    }
    fn from_binary<I: Iterator<Item = u8>>(source: &mut I) -> Option<Self> {
        let mut res = Vec::new();
        'grab_elements: loop {
            let mut bytes = [0; N];
            for (i, data) in (0..N).map(|_| source.next()).enumerate() {
                bytes[i] = data?;
            };
            if bytes == T::TERMINATOR {
                break 'grab_elements;
            };
            res.push(T::from_binary_in_vec(bytes)?);
        };
        Some(res)
    }
    fn from_binary_syntaxes<I: Iterator<Item = u8>>(source: &mut I, _syntaxes: &Vec<Syntax>) -> Option<Self> {
        Self::from_binary(source)
    }
}

impl<T, const N: usize> BinaryConvert<N> for Vec<Vec<T>>
where T: Vectorizable<BinaryForm = [u8; N]> {
    fn to_binary(self) -> Vec<u8> {
        let mut res =  Vec::new();
        for sub_vec in self {
            for element in sub_vec {
                res.extend_from_slice(&element.to_binary_in_vec());
            };
            res.extend_from_slice(&T::TERMINATOR.clone());
        };
        res.extend_from_slice(&T::TERMINATOR2.clone());
        res
    }
    fn from_binary<I: Iterator<Item = u8>>(source: &mut I) -> Option<Self> {
        let mut res = Vec::new();
        'grab_vectors: loop {
            let mut sub_res = Vec::new();
            'grab_elements: loop {
                let mut bytes = [0; N];
                for (i, data) in (0..N).map(|_| source.next()).enumerate() {
                    bytes[i] = data?;
                };
                if bytes == T::TERMINATOR {
                    res.push(sub_res);
                    break 'grab_elements;
                };
                if bytes == T::TERMINATOR2 {
                    break 'grab_vectors;
                };
                sub_res.push(T::from_binary_in_vec(bytes)?);
            };
        };
        Some(res)
    }
    fn from_binary_syntaxes<I: Iterator<Item = u8>>(source: &mut I, _syntaxes: &Vec<Syntax>) -> Option<Self> {
        Self::from_binary(source)
    }
}

impl BinaryConvert<0> for Syntax {
    fn to_binary(self) -> Vec<u8> {
        let Syntax {
            syntax_type,
            formula,
            distinct_wff_count,
            distinct_object_count
        } = self;
        let mut res = match syntax_type {
            SyntaxType::Formula => vec![0x00],
            SyntaxType::Object => vec![0x01]
        };
        res.append(&mut distinct_wff_count.to_binary());
        res.append(&mut distinct_object_count.to_binary());
        res.append(&mut formula.to_binary());
        res
    }
    fn from_binary<I: Iterator<Item = u8>>(source: &mut I) -> Option<Self> {
        let syntax_type = match source.next() {
            Some(0x00) => SyntaxType::Formula,
            Some(0x01) => SyntaxType::Object,
            _ => return None
        };
        let distinct_wff_count = usize::from_binary(source)?;
        let distinct_object_count = usize::from_binary(source)?;
        let formula = Vec::<Placeholder>::from_binary(source)?;
        Some(Syntax { syntax_type, formula, distinct_wff_count, distinct_object_count })
    }
    fn from_binary_syntaxes<I: Iterator<Item = u8>>(source: &mut I, _syntaxes: &Vec<Syntax>) -> Option<Self> {
        Self::from_binary(source)
    }
}

impl BinaryConvert<0> for Definition {
    fn to_binary(self) -> Vec<u8> {
        let Definition {
            name,
            definition,
            distinct_wff_count,
            distinct_object_count
        } = self;
        let definition_rpn = wff_to_rpn(definition);
        let mut res = Vec::new();
        res.append(&mut name.to_binary());
        res.append(&mut distinct_wff_count.to_binary());
        res.append(&mut distinct_object_count.to_binary());
        res.append(&mut definition_rpn.to_binary());
        res
    }
    fn from_binary<I>(_source: &mut I) -> Option<Self> { None }
    fn from_binary_syntaxes<I: Iterator<Item = u8>>(source: &mut I, syntaxes:&Vec<Syntax>) -> Option<Self> {
        let name = String::from_binary(source)?;
        let distinct_wff_count = usize::from_binary(source)?;
        let distinct_object_count = usize::from_binary(source)?;
        let definition_rpn = Vec::<RpnBlock>::from_binary(source)?;
        let definition = rpn_to_wff(definition_rpn, syntaxes)?;
        Some(Definition { name, definition, distinct_wff_count, distinct_object_count })
    }
}

impl BinaryConvert<0> for Axiom {
    fn to_binary(self) -> Vec<u8> {
        let Axiom {
            name,
            hypotheses,
            assertions,
            distinct_wff_count,
            distinct_object_count
        } = self;
        let hypots_rpn = hypotheses.into_iter().map(wff_to_rpn).collect::<Vec<_>>();
        let asserts_rpn = assertions.into_iter().map(wff_to_rpn).collect::<Vec<_>>();
        let mut res = Vec::new();
        res.append(&mut name.to_binary());
        res.append(&mut distinct_wff_count.to_binary());
        res.append(&mut distinct_object_count.to_binary());
        res.append(&mut hypots_rpn.to_binary());
        res.append(&mut asserts_rpn.to_binary());
        res
    }
    fn from_binary<I>(_source: &mut I) -> Option<Self> { None }
    fn from_binary_syntaxes<I: Iterator<Item = u8>>(source: &mut I, syntaxes: &Vec<Syntax>) -> Option<Self> {
        let name = String::from_binary(source)?;
        let distinct_wff_count = usize::from_binary(source)?;
        let distinct_object_count = usize::from_binary(source)?;
        let hypots_rpn = Vec::<Vec<RpnBlock>>::from_binary(source)?;
        let asserts_rpn = Vec::<Vec<RpnBlock>>::from_binary(source)?;
        let hypotheses = hypots_rpn.into_iter()
            .map(|hyp| rpn_to_wff(hyp, syntaxes))
            .collect::<Option<Vec<_>>>()?;
        let assertions = asserts_rpn.into_iter()
            .map(|hyp| rpn_to_wff(hyp, syntaxes))
            .collect::<Option<Vec<_>>>()?;
        Some(Axiom { name, hypotheses, assertions, distinct_wff_count, distinct_object_count })
    }
}

impl BinaryConvert<0> for Theorem {
    fn to_binary(self) -> Vec<u8> {
        let Theorem {
            name,
            hypotheses,
            assertions,
            proof,
            distinct_wff_count,
            distinct_object_count
        } = self;
        let hypots_rpn = hypotheses.into_iter().map(wff_to_rpn).collect::<Vec<_>>();
        let asserts_rpn = assertions.into_iter().map(wff_to_rpn).collect::<Vec<_>>();
        let (pr_hyps, pr_refs, pr_formulas) = transpose_steps(proof);
        let mut res = Vec::new();
        res.append(&mut name.to_binary());
        res.append(&mut distinct_wff_count.to_binary());
        res.append(&mut distinct_object_count.to_binary());
        res.append(&mut hypots_rpn.to_binary());
        res.append(&mut asserts_rpn.to_binary());
        res.append(&mut pr_hyps.to_binary());
        res.append(&mut pr_refs.to_binary());
        res.append(&mut pr_formulas.to_binary());
        res
    }
    fn from_binary<I>(_source: &mut I) -> Option<Self> { None }
    fn from_binary_syntaxes<I: Iterator<Item = u8>>(source: &mut I, syntaxes: &Vec<Syntax>) -> Option<Self> {
        let name = String::from_binary(source)?;
        let distinct_wff_count = usize::from_binary(source)?;
        let distinct_object_count = usize::from_binary(source)?;
        let hypots_rpn = Vec::<Vec<RpnBlock>>::from_binary(source)?;
        let asserts_rpn = Vec::<Vec<RpnBlock>>::from_binary(source)?;
        let pr_hyps = Vec::<Vec<usize>>::from_binary(source)?;
        let pr_refs = Vec::<Reference>::from_binary(source)?;
        let pr_formulas = Vec::<Vec<RpnBlock>>::from_binary(source)?;
        let hypotheses = hypots_rpn.into_iter()
            .map(|hyp| rpn_to_wff(hyp, syntaxes))
            .collect::<Option<Vec<_>>>()?;
        let assertions = asserts_rpn.into_iter()
            .map(|hyp| rpn_to_wff(hyp, syntaxes))
            .collect::<Option<Vec<_>>>()?;
        let proof = transpose_3vec(pr_hyps, pr_refs, pr_formulas, syntaxes)?;
        Some(Theorem { name, hypotheses, assertions, proof, distinct_wff_count, distinct_object_count })
    }
}

fn transpose_steps(proof: Vec<LogicStep>) -> (Vec<Vec<usize>>, Vec<Reference>, Vec<Vec<RpnBlock>>) {
    let mut hyps = Vec::new();
    let mut refs = Vec::new();
    let mut formulas = Vec::new();
    for step in proof {
        let LogicStep {
            used_hypotheses,
            theorem_ref,
            resulting_formula
        } = step;
        hyps.push(used_hypotheses);
        refs.push(theorem_ref);
        formulas.push(wff_to_rpn(resulting_formula))
    };
    (hyps, refs, formulas)
}

fn transpose_3vec(
    hyps: Vec<Vec<usize>>, refs: Vec<Reference>, formulas: Vec<Vec<RpnBlock>>, syntaxes: &Vec<Syntax>
) -> Option<Vec<LogicStep>> {
    let formulas = formulas.into_iter()
        .map(|formula_rpn| rpn_to_wff(formula_rpn, syntaxes))
        .collect::<Option<Vec<_>>>()?;
    let res = hyps.into_iter().zip(refs).zip(formulas)
        .map(|((used_hypotheses, theorem_ref), resulting_formula)|
            LogicStep {
                used_hypotheses,
                theorem_ref,
                resulting_formula
            } 
        ).collect::<Vec<_>>();
    Some(res)
}
