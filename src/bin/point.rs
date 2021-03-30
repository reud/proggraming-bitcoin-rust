use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
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
    return Point { x, y };
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

fn new_point_on_graph(point: Point, graph: PlanarGraph) -> Result<PointOnGraph, &'static str> {
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
