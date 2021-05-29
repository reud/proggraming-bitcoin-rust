use crate::scripts::helper::hash256;

#[derive(Debug,Clone,Ord, PartialOrd, Eq, PartialEq)]
pub struct Element {
    pub(crate) inner_data: Vec<u8>
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
    // 新たなオブジェクトを生成する。
    #[allow(dead_code)]
    pub fn reverse(&self) -> Element {
        let mut result = self.clone();
        result.inner_data.reverse();
        return result
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.inner_data.len()
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