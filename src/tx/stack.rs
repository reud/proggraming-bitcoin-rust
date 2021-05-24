use num_bigint::BigUint;

#[derive(Debug,Clone,Copy)]
pub struct Stack<T> {
    stack: Vec<T>
}

impl Stack<T> {
    pub fn push(mut self, element: T) {
        self.stack.push(element);
    }
    pub fn pop(mut self) -> Option<T> {
        self.stack.pop()
    }
}