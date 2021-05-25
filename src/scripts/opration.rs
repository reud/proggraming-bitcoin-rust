use crate::scripts::element::Element;
use crate::scripts::stack::Stack;

#[allow(dead_code)]
pub struct Operations {
}

impl Operations {
    #[allow(dead_code)]
    pub fn code_functions(code: u8) -> Option<fn(&mut Stack<Element>) -> bool> {
        return match code {
            0x76 => Some(Operations::op_dup),
            0xaa => Some(Operations::op_hash256),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn op_dup(stack: &mut Stack<Element>) -> bool {
        if stack.is_empty() {
            return false;
        }
        let top = stack.top().unwrap();
        stack.push(top);
        return true;
    }
    #[allow(dead_code)]
    pub fn op_hash256(stack: &mut Stack<Element>) -> bool {
        if stack.is_empty() {
            return false;
        }
        let top = stack.pop().unwrap();
        stack.push(top.hash256());
        return true;
    }
}