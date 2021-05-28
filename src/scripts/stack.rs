
#[derive(Debug,Clone)]
pub struct Stack<T> {
    stack: Vec<T>
}

impl<T: std::clone::Clone> Stack<T> {
    pub fn push(&mut self, element: T) {
        self.stack.push(element);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    pub fn top(&mut self) -> Option<T> {
        if self.stack.is_empty() {
            return None;
        }
        Some(self.stack[self.stack.len() - 1].clone())
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn len(&self) -> usize {
        self.stack.len()
    }
    pub fn get(&self, index: usize) -> Option<T> {
        self.stack.get(index)
    }
}

fn new_stack<T: Clone>() -> Stack<T> {
    let stack: Vec<T> = Vec::new();
    Stack {
        stack
    }
}

#[allow(dead_code)]
fn new_stack_with_default<T: Clone>( stack: &mut Vec<T>) -> Stack<T> {
    Stack {
        stack: stack.clone()
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::scripts::stack::{new_stack, Stack};

    #[test]
    fn test_stack() {
        {
            let mut s: Stack<i32> = new_stack();
            s.push(3);
            s.push(2);
            s.push(1);
            assert_eq!(s.pop().unwrap(),1);
            assert_eq!(s.pop().unwrap(),2);
            assert_eq!(s.pop().unwrap(),3);
        }
    }
}