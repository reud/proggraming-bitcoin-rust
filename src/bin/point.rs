use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Rem, Sub};

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
        if self.point.is_infinity {
            return rhs;
        }

        if rhs.point.is_infinity {
            return self;
        }

        let x3 = (((rhs.point.y - self.point.y) * (rhs.point.y - self.point.y))
            / ((rhs.point.x - self.point.x) * (rhs.point.x - self.point.x)))
            - self.point.x
            - rhs.point.x;
        let y3 = ((rhs.point.y - self.point.y) * (self.point.x - x3)
            / (rhs.point.x - self.point.x))
            - rhs.point.y;
        return new_point_on_graph(new_point(x3, y3), self.graph).unwrap();
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

fn main() {
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
            Err(e) => {}
        }
        let b = new_point_on_graph(new_point(-1, -1), g.clone());
        match b {
            Ok(v) => {
                println!("グラフ: {} 上に 点{}", v.graph.expression, v.point)
            }
            Err(e) => {}
        }

        let c = new_point_on_graph(new_point(18, 77), g.clone());
        match c {
            Ok(v) => {
                println!("グラフ: {} 上に 点{}", v.graph.expression, v.point)
            }
            Err(e) => {}
        }
        let d = new_point_on_graph(new_point(5, 7), g);
        match d {
            Ok(v) => {
                println!("グラフ: {} 上に 点{}", v.graph.expression, v.point)
            }
            Err(e) => {}
        }
    }
}
