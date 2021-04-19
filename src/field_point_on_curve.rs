use crate::field_element;
use crate::field_graph;
use crate::field_graph::FieldPlanarGraph;
use crate::field_point;
use crate::field_point::{ new_point, FieldPoint};
use std::ops::Add;

pub fn new_field_point_on_graph(
    point: field_point::FieldPoint,
    graph: field_graph::FieldPlanarGraph,
) -> Result<FieldPointOnGraph, &'static str> {
    if point.is_infinity {
        return Ok(FieldPointOnGraph { graph, point });
    }

    // 点がグラフ上にあること
    if (graph.lhs)(point.y) != (graph.rhs)(point.x) {
        return Err("point not on curve");
    }

    return Ok(FieldPointOnGraph { graph, point });
}

#[derive(Debug, Clone)]
pub struct FieldPointOnGraph {
    pub graph: FieldPlanarGraph,
    pub point: FieldPoint,
}
impl PartialEq for FieldPointOnGraph {
    fn eq(&self, other: &Self) -> bool {
        return self.point == other.point && self.graph == other.graph;
    }
}

impl Add for FieldPointOnGraph {
    type Output = FieldPointOnGraph;
    fn add(self, rhs: Self) -> Self::Output {
        if self.graph != rhs.graph {
            panic!(
                "graph mismatch. can't add function with \n lhs graph: {} \n rhs graph: {}",
                self.graph.expression, rhs.graph.expression
            );
        }

        if self.point.is_infinity {
            return rhs;
        }

        if rhs.point.is_infinity {
            return self;
        }

        // 楕円曲線の座標が一致する時は接点の傾きを利用する。
        if self == rhs {
            let prime = self.point.x.prime; // 適当に素数を持ってくる
            let f0 = field_element::new_field_element(0, prime);
            let f1 = field_element::new_field_element(1, prime);
            let f2 = field_element::new_field_element(2, prime);
            let f3 = field_element::new_field_element(3, prime);
            if self.point.y == f0 && rhs.point.y == f0 {
                return new_field_point_on_graph(field_point::new_empty_point(prime), self.graph)
                    .unwrap();
            }

            let b = (self.graph.rhs)(f0);
            let a = (self.graph.rhs)(f1) - f1 - b;
            let x = self.point.x;
            let y = self.point.y;
            let s = (f3 * x * x + a) / (f2 * y);
            let x3 = s * s - (f2 * x);
            let y3 = s * (x - x3) - y;
            let p = new_point(x3, y3);
            return new_field_point_on_graph(p, self.graph).unwrap();
        }

        // 加法逆元の場合、無限遠点を返す
        let prime = self.point.x.prime; // 適当に素数を持ってくる
        let f0 = field_element::new_field_element(0, prime);
        if self.point.x == rhs.point.x && (self.point.y + rhs.point.y) == f0 {
            let prime = self.point.x.prime; // 適当に素数を持ってくる
            return new_field_point_on_graph(field_point::new_empty_point(prime), self.graph).unwrap();
        }

        let s = (rhs.point.y - self.point.y) / (rhs.point.x - self.point.x);
        let x3 = s * s - self.point.x - rhs.point.x;
        let y3 = s * (self.point.x - x3) - self.point.y;

        let p = new_point(x3, y3);
        return new_field_point_on_graph(p, self.graph).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_graph::new_field_planar_graph;
    use crate::field_point_on_curve;

    #[test]
    fn test_add() {
        // P.49 練習問題2,3
        {
            let field = 223;
            let lhs = |y: field_element::FieldElement| {
                return y * y;
            };
            let rhs = |x: field_element::FieldElement| {
                return x * x * x + field_element::new_field_element(7,x.prime);
            };
            let graph = new_field_planar_graph(lhs,rhs,field,"y^2 = x^3 + 7");

            {
                let x1 = field_element::new_field_element(170,223);
                let y1 = field_element::new_field_element(142,223);
                let x2 = field_element::new_field_element(60,223);
                let y2 = field_element::new_field_element(139,223);
                let x3 = field_element::new_field_element(220,223);
                let y3 = field_element::new_field_element(181,223);

                let g1 = graph.clone();
                let g2 = graph.clone();
                let g3 = graph.clone();

                let left_point = field_point::new_point(x1,y1);
                let left_point_on_graph = field_point_on_curve::new_field_point_on_graph(left_point,g1)
                    .unwrap();

                let right_point = field_point::new_point(x2,y2);
                let right_point_on_graph = field_point_on_curve::new_field_point_on_graph(right_point,g2)
                    .unwrap();

                let expected_point = field_point::new_point(x3,y3);
                let expected_point_on_graph = field_point_on_curve::new_field_point_on_graph(expected_point,g3)
                    .unwrap();

                let sum = left_point_on_graph+right_point_on_graph;
                assert_eq!(expected_point_on_graph,sum);
                println!("F: {} 楕円曲線y^2 = x^3 + 7 上での 点{} + {} = {}",field,left_point,right_point,sum.point);
            }

            {
                let x1 = field_element::new_field_element(47,223);
                let y1 = field_element::new_field_element(71,223);
                let x2 = field_element::new_field_element(17,223);
                let y2 = field_element::new_field_element(56,223);
                let x3 = field_element::new_field_element(215,223);
                let y3 = field_element::new_field_element(68,223);

                let g1 = graph.clone();
                let g2 = graph.clone();
                let g3 = graph.clone();

                let left_point = field_point::new_point(x1,y1);
                let left_point_on_graph = field_point_on_curve::new_field_point_on_graph(left_point,g1)
                    .unwrap();

                let right_point = field_point::new_point(x2,y2);
                let right_point_on_graph = field_point_on_curve::new_field_point_on_graph(right_point,g2)
                    .unwrap();

                let expected_point = field_point::new_point(x3,y3);
                let expected_point_on_graph = field_point_on_curve::new_field_point_on_graph(expected_point,g3)
                    .unwrap();

                let sum = left_point_on_graph+right_point_on_graph;
                assert_eq!(expected_point_on_graph,sum);
                println!("F: {} 楕円曲線y^2 = x^3 + 7 上での 点{} + {} = {}",field,left_point,right_point,sum.point);
            }

            {
                let x1 = field_element::new_field_element(143,223);
                let y1 = field_element::new_field_element(98,223);
                let x2 = field_element::new_field_element(76,223);
                let y2 = field_element::new_field_element(66,223);
                let x3 = field_element::new_field_element(47,223);
                let y3 = field_element::new_field_element(71,223);

                let g1 = graph.clone();
                let g2 = graph.clone();
                let g3 = graph.clone();

                let left_point = field_point::new_point(x1,y1);
                let left_point_on_graph = field_point_on_curve::new_field_point_on_graph(left_point,g1)
                    .unwrap();

                let right_point = field_point::new_point(x2,y2);
                let right_point_on_graph = field_point_on_curve::new_field_point_on_graph(right_point,g2)
                    .unwrap();

                let expected_point = field_point::new_point(x3,y3);
                let expected_point_on_graph = field_point_on_curve::new_field_point_on_graph(expected_point,g3)
                    .unwrap();

                let sum = left_point_on_graph+right_point_on_graph;
                assert_eq!(expected_point_on_graph,sum);
                println!("F: {} 楕円曲線y^2 = x^3 + 7 上での 点{} + {} = {}",field,left_point,right_point,sum.point);
            }
        }
    }
}