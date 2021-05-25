use crate::scripts::helper::hash256;

#[derive(Debug,Clone)]
pub struct Element {
    inner_data: Vec<u8>
}

impl Element {
    #[allow(dead_code)]
    pub fn hash256(self) -> Element {
        return Element {
            inner_data: hash256(self.inner_data)
        }
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        return self.inner_data.is_empty()
    }
}
pub fn new_element() -> Element {
    Element {
        inner_data: vec![]
    }
}

pub fn new_element_from_bytes(bytes: Vec<u8>) -> Element {
    Element {
        inner_data: bytes
    }
}