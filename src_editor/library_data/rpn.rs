use super::{
    Syntax,
    WellFormedFormula, Object
};

pub enum RpnBlock {
    WffAtomic(usize),
    WffComposite(usize),
    ObjectAtomic(usize),
    ObjectComposite(usize)
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
