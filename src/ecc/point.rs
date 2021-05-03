use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    is_infinity: bool,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_infinity {
          return write!(f,"(無限遠点)");
        }
        write!(f, "({},{})", self.x, self.y)
    }
}

fn new_point(x: i64, y: i64) -> Point {
    return Point {
        x,
        y,
        is_infinity: false,
    };
}

fn new_empty_point() -> Point {
    return Point {
        x: -1,
        y: -1,
        is_infinity: true,
    };
}

#[derive(Debug, Clone)]
struct PlanarGraph {
    lhs: fn(y: i64) -> i64,
    rhs: fn(x: i64) -> i64,
    expression: String, // 式(一致に利用する)
}

impl PlanarGraph {
    #[allow(dead_code)]
    pub fn is_on_curve(self, point: Point) -> bool {
        return (self.lhs)(point.y) == (self.rhs)(point.x);
    }
}

fn new_planar_graph(
    lhs: fn(y: i64) -> i64,
    rhs: fn(x: i64) -> i64,
    exp: impl Into<String>,
) -> PlanarGraph {
    return PlanarGraph {
        lhs,
        rhs,
        expression: exp.into(),
    };
}

impl PartialEq for PlanarGraph {
    fn eq(&self, other: &Self) -> bool {
        // 雑チェック
        return (self.lhs)(0) == (other.lhs)(0)
            && (self.rhs)(0) == (other.rhs)(0)
            && self.expression == other.expression;
    }
}

#[derive(Debug, Clone)]
struct PointOnGraph {
    graph: PlanarGraph,
    point: Point,
}

impl PartialEq for PointOnGraph {
    fn eq(&self, other: &Self) -> bool {
        return self.point == other.point && self.graph == other.graph;
    }
}

impl Add for PointOnGraph {
    type Output = PointOnGraph;

    fn add(self, rhs: Self) -> Self::Output {

        if self.graph != rhs.graph {
            panic!("graph mismatch. can't add function with \n lhs graph: {} \n rhs graph: {}", self.graph.expression,rhs.graph.expression);
        }

        if self.point.is_infinity {
            return rhs;
        }

        if rhs.point.is_infinity {
            return self;
        }

        // 楕円曲線の座標が一致する時は接点の傾きを利用する。
        if self == rhs {
            if self.point.y == 0 && rhs.point.y == 0 {
                return new_point_on_graph(new_empty_point(),self.graph).unwrap();
            }

            let b = (self.graph.rhs)(0);
            let a = (self.graph.rhs)(1) - 1 - b;
            let x = self.point.x as f64;
            let y = self.point.y as f64;
            let s = (3.0 * x * x + a as f64) / (2.0 * y);
            let x3 = s * s - 2.0 * x;
            let y3 = s * (x - x3) - y;
            let p = new_point(x3 as i64, y3 as i64);
            return new_point_on_graph(p,self.graph).unwrap();
        }

        // 加法逆元の場合、無限遠点を返す
        if self.point.x == rhs.point.x && self.point.y == -rhs.point.y {
            return new_point_on_graph(new_empty_point(),self.graph).unwrap();
        }

        let s = (rhs.point.y - self.point.y) as f64 / (rhs.point.x - self.point.x) as f64;
        let x3 = s * s - self.point.x as f64 - rhs.point.x as f64;
        let y3 = s * (self.point.x as f64 - x3) - self.point.y as f64;

        let p = new_point(x3 as i64, y3 as i64);
        return new_point_on_graph(p, self.graph).unwrap();
    }
}

fn new_point_on_graph(point: Point, graph: PlanarGraph) -> Result<PointOnGraph, &'static str> {
    if point.is_infinity {
        return Ok(PointOnGraph { graph, point });
    }

    // 点がグラフ上にあること
    if (graph.lhs)(point.y) != (graph.rhs)(point.x) {
        return Err("point not on curve");
    }

    return Ok(PointOnGraph { graph, point });
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    #[test]
    fn test_point_all() {
        // P.28 練習問題1
        {
            println!("P.28 Q1");
            let g = new_planar_graph(
                |y| -> i64 { return y * y },
                |x| return x * x * x + 5 * x + 7,
                "y^2=x^3+5x+7",
            );

            let a = new_point_on_graph(new_point(2, 4), g.clone());
            match a {
                Ok(v) => {
                    println!("グラフ: {} 上に 点{}", v.graph.expression, v.point)
                }
                Err(_e) => {}
            }
            let b = new_point_on_graph(new_point(-1, -1), g.clone());
            match b {
                Ok(v) => {
                    println!("グラフ: {} 上に 点{}", v.graph.expression, v.point)
                }
                Err(_e) => {}
            }

            let c = new_point_on_graph(new_point(18, 77), g.clone());
            match c {
                Ok(v) => {
                    println!("グラフ: {} 上に 点{}", v.graph.expression, v.point)
                }
                Err(_e) => {}
            }
            let d = new_point_on_graph(new_point(5, 7), g);
            match d {
                Ok(v) => {
                    println!("グラフ: {} 上に 点{}", v.graph.expression, v.point)
                }
                Err(_e) => {}
            }
        }
        // P.28 練習問題2
        {
            let g = new_planar_graph(
                |y| -> i64 { return y * y },
                |x| return x * x * x + 5 * x + 7,
                "y^2=x^3+5x+7",
            );
            let a = new_point_on_graph(new_point(-1, -1), g.clone()).unwrap();
            let b = new_point_on_graph(new_point(18, 77), g.clone()).unwrap();

            println!("点a {}と点b {} について、a != b は {}",a.point,b.point, a != b)
        }
        // P.35  練習問題3
        {
            let g = new_planar_graph(
                |y| -> i64 { return y * y },
                |x| return x * x * x + 5 * x + 7,
                "y^2=x^3+5x+7",
            );
            let a = new_point_on_graph(new_point(-1, -1), g.clone()).unwrap();
            let b = new_point_on_graph(new_point(-1, 1), g.clone()).unwrap();

            let sum = a.clone() + b.clone();

            println!("加法逆元の関係である、点a {}と 点b{} について、a + b は {}",a.point,b.point, sum.point)
        }
        // P.37,38 練習問題4,5
        {
            let g = new_planar_graph(
                |y| -> i64 { return y * y },
                |x| return x * x * x + 5 * x + 7,
                "y^2=x^3+5x+7",
            );
            let a = new_point_on_graph(new_point(2, 5), g.clone()).unwrap();
            let b = new_point_on_graph(new_point(-1, -1), g.clone()).unwrap();

            let sum = a.clone() + b.clone();

            println!("点a {}と 点b{} について、a + b は {}",a.point,b.point, sum.point)
        }
        // P40 練習問題6,7
        {
            let g = new_planar_graph(
                |y| -> i64 { return y * y },
                |x| return x * x * x + 5 * x + 7,
                "y^2=x^3+5x+7",
            );
            let a = new_point_on_graph(new_point(-1, -1), g.clone()).unwrap();

            let sum = a.clone() + a.clone();
            println!("点a + 点a: {} = {}", a.point.clone(), sum.point);
        }
    }
}

