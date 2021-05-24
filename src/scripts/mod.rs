use crate::scripts::element::Element;

mod element;
mod opration;
mod stack;


pub struct ScriptStack {
    inner_stack: Vec<Element>
}

impl ScriptStack {
    pub fn push(mut self, el: Element) {
        self.inner_stack.push(el);
    }
    pub fn size(self) -> usize {
        return self.inner_stack.len()
    }
}