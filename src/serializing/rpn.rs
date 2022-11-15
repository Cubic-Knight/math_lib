use crate::compiling::{
    Syntax,
    WellFormedFormula, Object
};

pub enum RpnBlock {
    WffAtomic(usize),
    WffComposite(usize),
    ObjectAtomic(usize),
    ObjectComposite(usize)
}

pub fn wff_to_rpn(wff: WellFormedFormula) -> Vec<RpnBlock> {
    let mut res = Vec::new();
    __wff_to_rpn(wff, &mut res);
    res
}

fn __wff_to_rpn(wff: WellFormedFormula, res: &mut Vec<RpnBlock>) {
    match wff {
        WellFormedFormula::Atomic(id) => res.push(RpnBlock::WffAtomic(id)),
        WellFormedFormula::SyntaxComposite {
            syntax_ref,
            wff_parameters,
            object_parameters
        } => {
            for param_wff in wff_parameters {
                __wff_to_rpn(param_wff, res);
            };
            for param_obj in object_parameters {
                __obj_to_rpn(param_obj, res);
            };
            res.push(RpnBlock::WffComposite(syntax_ref))
        }
    }
}

fn __obj_to_rpn(obj: Object, res: &mut Vec<RpnBlock>) {
    match obj {
        Object::Atomic(id) => res.push(RpnBlock::ObjectAtomic(id)),
        Object::SyntaxComposite {
            syntax_ref,
            wff_parameters,
            object_parameters
        } => {
            for param_wff in wff_parameters {
                __wff_to_rpn(param_wff, res);
            };
            for param_obj in object_parameters {
                __obj_to_rpn(param_obj, res);
            };
            res.push(RpnBlock::ObjectComposite(syntax_ref))
        }
    }
}

pub fn rpn_to_wff(rpn: Vec<RpnBlock>, syntaxes: &Vec<Syntax>) -> Option<WellFormedFormula> {
    let mut wff_stack = Vec::new();
    let mut obj_stack = Vec::new();
    for block in rpn {
        match block {
            RpnBlock::WffAtomic(id) => wff_stack.push(WellFormedFormula::Atomic(id)),
            RpnBlock::ObjectAtomic(id) => obj_stack.push(Object::Atomic(id)),
            RpnBlock::WffComposite(syntax_ref) => {
                let &Syntax {
                    syntax_type: _,
                    formula: _,
                    distinct_wff_count: wffc,
                    distinct_object_count: objc
                } = syntaxes.get(syntax_ref)?;
                let new_wff_stack_len = wff_stack.len().checked_sub(wffc)?;
                let new_obj_stack_len = obj_stack.len().checked_sub(objc)?;
                let wff = WellFormedFormula::SyntaxComposite {
                    syntax_ref,
                    wff_parameters: wff_stack.split_off(new_wff_stack_len),
                    object_parameters: obj_stack.split_off(new_obj_stack_len)
                };
                wff_stack.push(wff);
            },
            RpnBlock::ObjectComposite(syntax_ref) => {
                let &Syntax {
                    syntax_type: _,
                    formula: _,
                    distinct_wff_count: wffc,
                    distinct_object_count: objc
                } = syntaxes.get(syntax_ref)?;
                let new_wff_stack_len = wff_stack.len().checked_sub(wffc)?;
                let new_obj_stack_len = obj_stack.len().checked_sub(objc)?;
                let obj = Object::SyntaxComposite {
                    syntax_ref,
                    wff_parameters: wff_stack.split_off(new_wff_stack_len),
                    object_parameters: obj_stack.split_off(new_obj_stack_len)
                };
                obj_stack.push(obj);
            }
        }
    }
    wff_stack.pop()
}
