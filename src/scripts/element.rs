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
}