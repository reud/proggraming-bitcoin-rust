use crate::field_point;
use crate::field_element;

#[derive(Debug, Clone)]
pub struct FieldPlanarGraph {
    lhs: fn(y: field_element::FieldElement) -> field_element::FieldElement,
    rhs: fn(x: field_element::FieldElement) -> field_element::FieldElement,
    field: u64,
    expression: String, // 式(一致に利用する)
}

impl FieldPlanarGraph {
    pub fn on_curve(self, point: field_point::FieldPoint) -> bool {
        if self.field != point.x.prime || self.field != point.y.prime {
            panic!("bad field value");
        }
        return (self.lhs)(point.y) == (self.rhs)(point.x);
    }
}

impl PartialEq for FieldPlanarGraph {
    fn eq(&self, other: &Self) -> bool {
        let f = field_element::new_field_element(1,self.field);
        // 雑チェック
        return (self.lhs)(f) == (other.lhs)(f)
            && (self.rhs)(f) == (other.rhs)(f)
            && self.expression == other.expression;
    }
}

pub fn new_field_planar_graph(
    lhs: fn(y: field_element::FieldElement) -> field_element::FieldElement,
    rhs: fn(y: field_element::FieldElement) -> field_element::FieldElement,
    field: u64,
    exp: impl Into<String>,
) -> FieldPlanarGraph {
    return FieldPlanarGraph {
        lhs,
        rhs,
        field,
        expression: exp.into(),
    };
}