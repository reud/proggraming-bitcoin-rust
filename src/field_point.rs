use crate::field_element;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct FieldPoint {
    pub(crate) x: field_element::FieldElement,
    pub(crate) y: field_element::FieldElement,
    pub(crate) is_infinity: bool,
}

impl PartialEq for FieldPoint {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Display for FieldPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_infinity {
            return write!(f,"(無限遠点)");
        }
        write!(f, "({},{})", self.x, self.y)
    }
}

pub fn new_point(x: field_element::FieldElement, y: field_element::FieldElement) -> FieldPoint {
    return FieldPoint {
        x,
        y,
        is_infinity: false,
    };
}

pub fn new_empty_point(prime: u64) -> FieldPoint {
    return FieldPoint {
        x: field_element::new_field_element(1,prime),
        y: field_element::new_field_element(1,prime),
        is_infinity: true,
    };
}

