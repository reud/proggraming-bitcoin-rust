use crate::field_point;
use crate::field_element;

#[derive(Debug, Clone)]
pub struct FieldPlanarGraph {
    pub(crate) lhs: fn(y: field_element::FieldElement) -> field_element::FieldElement,
    pub(crate) rhs: fn(x: field_element::FieldElement) -> field_element::FieldElement,
    field: u64,
    pub(crate) expression: String, // 式(一致に利用する)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_graph::new_field_planar_graph;
    use crate::{field_point_on_curve, field_graph};

    #[test]
    fn test_on_curve() {
        // P.45 練習問題1
        {
            let lhs = |y: field_element::FieldElement| {
                return y * y;
            };
            let rhs = |x: field_element::FieldElement| {
                let seven = field_element::new_field_element(7,x.prime);
                return (x*x*x)+seven;
            };
            {
                let g = field_graph::new_field_planar_graph(lhs,rhs,223,"y^2 = x^3 + 7");
                let x = field_element::new_field_element(192,223);
                let y = field_element::new_field_element(105,223);
                let point = field_point::new_point(x,y);
                let msg = if g.on_curve(point) { "曲線上にある。" } else { "曲線上にない。" };
                println!("F223 y^2 = x^3 + 7 において 点{} は{}",point,msg);
            }
            {
                let g = field_graph::new_field_planar_graph(lhs,rhs,223,"y^2 = x^3 + 7");
                let x = field_element::new_field_element(17,223);
                let y = field_element::new_field_element(56,223);
                let point = field_point::new_point(x,y);
                let msg = if g.on_curve(point) { "曲線上にある。" } else { "曲線上にない。" };
                println!("F223 y^2 = x^3 + 7 において 点{} は{}",point,msg);
            }
            {
                let g = field_graph::new_field_planar_graph(lhs,rhs,223,"y^2 = x^3 + 7");
                let x = field_element::new_field_element(200,223);
                let y = field_element::new_field_element(119,223);
                let point = field_point::new_point(x,y);
                let msg = if g.on_curve(point) { "曲線上にある。" } else { "曲線上にない。" };
                println!("F223 y^2 = x^3 + 7 において 点{} は{}",point,msg);
            }
            {
                let g = field_graph::new_field_planar_graph(lhs,rhs,223,"y^2 = x^3 + 7");
                let x = field_element::new_field_element(1,223);
                let y = field_element::new_field_element(193,223);
                let point = field_point::new_point(x,y);
                let msg = if g.on_curve(point) { "曲線上にある。" } else { "曲線上にない。" };
                println!("F223 y^2 = x^3 + 7 において 点{} は{}",point,msg);
            }
            {
                let g = field_graph::new_field_planar_graph(lhs,rhs,223,"y^2 = x^3 + 7");
                let x = field_element::new_field_element(42,223);
                let y = field_element::new_field_element(99,223);
                let point = field_point::new_point(x,y);
                let msg = if g.on_curve(point) { "曲線上にある。" } else { "曲線上にない。" };
                println!("F223 y^2 = x^3 + 7 において 点{} は{}",point,msg);
            }
        }

    }
}