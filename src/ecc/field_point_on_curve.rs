use crate::ecc::field_element;
use crate::ecc::field_graph;
use crate::ecc::field_graph::FieldPlanarGraph;
use crate::ecc::field_point;
use crate::ecc::field_point::{new_empty_point, new_point, FieldPoint};
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

impl FieldPointOnGraph {
    fn inner_mul(self, f: FieldPointOnGraph, v: u128) -> FieldPointOnGraph {
        let prime = self.point.x.prime;
        if v == 0 {
            return new_field_point_on_graph(new_empty_point(prime), self.graph).unwrap();
        }
        if v % 2 == 0 {
            let half_res = self.inner_mul(f, v / 2).clone();
            let half_res2 = half_res.clone();
            return half_res + half_res2;
        }
        let cf = f.clone();
        f.clone() + f.inner_mul(cf, v - 1)
    }
    fn mul(self, v: u128) -> FieldPointOnGraph {
        let this = self.clone();
        self.inner_mul(this, v)
    }
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
            return new_field_point_on_graph(field_point::new_empty_point(prime), self.graph)
                .unwrap();
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
    extern crate test;
    use super::*;
    use crate::ecc::field_graph::new_field_planar_graph;
    use crate::ecc::field_point_on_curve;
    use test::Bencher;

    #[test]
    fn test_add() {
        // P.49 練習問題2,3
        {
            let field = 223;
            let lhs = |y: field_element::FieldElement| {
                return y * y;
            };
            let rhs = |x: field_element::FieldElement| {
                return x * x * x + field_element::new_field_element(7, x.prime);
            };
            let graph = new_field_planar_graph(lhs, rhs, field, "y^2 = x^3 + 7");

            {
                let x1 = field_element::new_field_element(170, 223);
                let y1 = field_element::new_field_element(142, 223);
                let x2 = field_element::new_field_element(60, 223);
                let y2 = field_element::new_field_element(139, 223);
                let x3 = field_element::new_field_element(220, 223);
                let y3 = field_element::new_field_element(181, 223);

                let g1 = graph.clone();
                let g2 = graph.clone();
                let g3 = graph.clone();

                let left_point = field_point::new_point(x1, y1);
                let left_point_on_graph =
                    field_point_on_curve::new_field_point_on_graph(left_point, g1).unwrap();

                let right_point = field_point::new_point(x2, y2);
                let right_point_on_graph =
                    field_point_on_curve::new_field_point_on_graph(right_point, g2).unwrap();

                let expected_point = field_point::new_point(x3, y3);
                let expected_point_on_graph =
                    field_point_on_curve::new_field_point_on_graph(expected_point, g3).unwrap();

                let sum = left_point_on_graph + right_point_on_graph;
                assert_eq!(expected_point_on_graph, sum);
                println!(
                    "F: {} 楕円曲線y^2 = x^3 + 7 上での 点{} + {} = {}",
                    field, left_point, right_point, sum.point
                );
            }

            {
                let x1 = field_element::new_field_element(47, 223);
                let y1 = field_element::new_field_element(71, 223);
                let x2 = field_element::new_field_element(17, 223);
                let y2 = field_element::new_field_element(56, 223);
                let x3 = field_element::new_field_element(215, 223);
                let y3 = field_element::new_field_element(68, 223);

                let g1 = graph.clone();
                let g2 = graph.clone();
                let g3 = graph.clone();

                let left_point = field_point::new_point(x1, y1);
                let left_point_on_graph =
                    field_point_on_curve::new_field_point_on_graph(left_point, g1).unwrap();

                let right_point = field_point::new_point(x2, y2);
                let right_point_on_graph =
                    field_point_on_curve::new_field_point_on_graph(right_point, g2).unwrap();

                let expected_point = field_point::new_point(x3, y3);
                let expected_point_on_graph =
                    field_point_on_curve::new_field_point_on_graph(expected_point, g3).unwrap();

                let sum = left_point_on_graph + right_point_on_graph;
                assert_eq!(expected_point_on_graph, sum);
                println!(
                    "F: {} 楕円曲線y^2 = x^3 + 7 上での 点{} + {} = {}",
                    field, left_point, right_point, sum.point
                );
            }

            {
                let x1 = field_element::new_field_element(143, 223);
                let y1 = field_element::new_field_element(98, 223);
                let x2 = field_element::new_field_element(76, 223);
                let y2 = field_element::new_field_element(66, 223);
                let x3 = field_element::new_field_element(47, 223);
                let y3 = field_element::new_field_element(71, 223);

                let g1 = graph.clone();
                let g2 = graph.clone();
                let g3 = graph.clone();

                let left_point = field_point::new_point(x1, y1);
                let left_point_on_graph =
                    field_point_on_curve::new_field_point_on_graph(left_point, g1).unwrap();

                let right_point = field_point::new_point(x2, y2);
                let right_point_on_graph =
                    field_point_on_curve::new_field_point_on_graph(right_point, g2).unwrap();

                let expected_point = field_point::new_point(x3, y3);
                let expected_point_on_graph =
                    field_point_on_curve::new_field_point_on_graph(expected_point, g3).unwrap();

                let sum = left_point_on_graph + right_point_on_graph;
                assert_eq!(expected_point_on_graph, sum);
                println!(
                    "F: {} 楕円曲線y^2 = x^3 + 7 上での 点{} + {} = {}",
                    field, left_point, right_point, sum.point
                );
            }
        }
    }

    #[test]
    fn test_mul() {
        // P.52 練習問題4
        let field = 223;
        let lhs = |y: field_element::FieldElement| {
            return y * y;
        };
        let rhs = |x: field_element::FieldElement| {
            return x * x * x + field_element::new_field_element(7, x.prime);
        };
        let graph = new_field_planar_graph(lhs, rhs, field, "y^2 = x^3 + 7");

        {
            let x = field_element::new_field_element(192, 223);
            let y = field_element::new_field_element(105, 223);
            let p = field_point::new_point(x, y);
            let g = graph.clone();
            let gp = field_point_on_curve::new_field_point_on_graph(p, g).unwrap();
            let gp = gp.mul(2);

            let exp_x = field_element::new_field_element(49, 223);
            let exp_y = field_element::new_field_element(71, 223);
            let exp_p = field_point::new_point(exp_x, exp_y);
            let g2 = graph.clone();
            let exp_gp = field_point_on_curve::new_field_point_on_graph(exp_p, g2).unwrap();
            assert_eq!(gp, exp_gp);
        }
        {
            let x = field_element::new_field_element(143, 223);
            let y = field_element::new_field_element(98, 223);
            let p = field_point::new_point(x, y);
            let g = graph.clone();
            let gp = field_point_on_curve::new_field_point_on_graph(p, g).unwrap();
            let gp = gp.mul(2);

            let exp_x = field_element::new_field_element(64, 223);
            let exp_y = field_element::new_field_element(168, 223);
            let exp_p = field_point::new_point(exp_x, exp_y);
            let g2 = graph.clone();
            let exp_gp = field_point_on_curve::new_field_point_on_graph(exp_p, g2).unwrap();
            assert_eq!(gp, exp_gp);
        }
        {
            let x = field_element::new_field_element(47, 223);
            let y = field_element::new_field_element(71, 223);
            let p = field_point::new_point(x, y);
            let g = graph.clone();
            let gp = field_point_on_curve::new_field_point_on_graph(p, g).unwrap();
            let gp = gp.mul(2);

            let exp_x = field_element::new_field_element(36, 223);
            let exp_y = field_element::new_field_element(111, 223);
            let exp_p = field_point::new_point(exp_x, exp_y);
            let g2 = graph.clone();
            let exp_gp = field_point_on_curve::new_field_point_on_graph(exp_p, g2).unwrap();
            assert_eq!(gp, exp_gp);
        }
        {
            let x = field_element::new_field_element(47, 223);
            let y = field_element::new_field_element(71, 223);
            let p = field_point::new_point(x, y);
            let g = graph.clone();
            let gp = field_point_on_curve::new_field_point_on_graph(p, g).unwrap();
            let gp = gp.mul(4);

            let exp_x = field_element::new_field_element(194, 223);
            let exp_y = field_element::new_field_element(51, 223);
            let exp_p = field_point::new_point(exp_x, exp_y);
            let g2 = graph.clone();
            let exp_gp = field_point_on_curve::new_field_point_on_graph(exp_p, g2).unwrap();
            assert_eq!(gp, exp_gp);
        }
        {
            let x = field_element::new_field_element(47, 223);
            let y = field_element::new_field_element(71, 223);
            let p = field_point::new_point(x, y);
            let g = graph.clone();
            let gp = field_point_on_curve::new_field_point_on_graph(p, g).unwrap();
            let gp = gp.mul(8);

            let exp_x = field_element::new_field_element(116, 223);
            let exp_y = field_element::new_field_element(55, 223);
            let exp_p = field_point::new_point(exp_x, exp_y);
            let g2 = graph.clone();
            let exp_gp = field_point_on_curve::new_field_point_on_graph(exp_p, g2).unwrap();
            assert_eq!(gp, exp_gp);
        }
        for i in 1..22 {
            let x = field_element::new_field_element(47, 223);
            let y = field_element::new_field_element(71, 223);
            let p = field_point::new_point(x, y);
            let g = graph.clone();
            let gp = field_point_on_curve::new_field_point_on_graph(p, g).unwrap();
            let gp = gp.mul(i);
            println!("{}: {}", i, gp.point);
        }
        {
            let x = field_element::new_field_element(47, 223);
            let y = field_element::new_field_element(71, 223);
            let p = field_point::new_point(x, y);
            let g = graph.clone();
            let gp = field_point_on_curve::new_field_point_on_graph(p, g).unwrap();
            let gp = gp.mul(21);

            let exp_p = field_point::new_empty_point(223);
            let g2 = graph.clone();
            let exp_gp = field_point_on_curve::new_field_point_on_graph(exp_p, g2).unwrap();
            assert_eq!(gp, exp_gp);
        }
        // P.58 練習問題5
        for i in 1..223 {
            let x = field_element::new_field_element(15, 223);
            let y = field_element::new_field_element(86, 223);
            let p = field_point::new_point(x, y);
            let g = graph.clone();
            let gp = field_point_on_curve::new_field_point_on_graph(p, g).unwrap();
            let gp = gp.mul(i);

            let exp_p = field_point::new_empty_point(223);
            let g2 = graph.clone();
            let exp_gp = field_point_on_curve::new_field_point_on_graph(exp_p, g2).unwrap();
            if gp == exp_gp {
                println!("{}回の乗算で無限遠点に。　位数は{}", i - 1, i);
                break;
            }
        }
    }

    #[bench]
    fn bench_mul(b: &mut Bencher) {
        let field = 223;
        let lhs = |y: field_element::FieldElement| {
            return y * y;
        };
        let rhs = |x: field_element::FieldElement| {
            return x * x * x + field_element::new_field_element(7, x.prime);
        };
        let graph = new_field_planar_graph(lhs, rhs, field, "y^2 = x^3 + 7");
        b.iter(|| {
            let x = field_element::new_field_element(15, 223);
            let y = field_element::new_field_element(86, 223);
            let _p = field_point::new_point(x, y);
            let _g = graph.clone();

            let exp_p = field_point::new_empty_point(223);
            let g2 = graph.clone();
            let _exp_gp = field_point_on_curve::new_field_point_on_graph(exp_p, g2).unwrap();
        });
    }
}
