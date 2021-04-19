use crate::field_graph::new_field_planar_graph;

mod field_point;
mod field_element;
mod point;
mod field_graph;
mod field_point_on_curve;

fn main() {
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
    // P.49 練習問題2
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
            let g1 = graph.clone();
            let g2 = graph.clone();

            let left_point = field_point::new_point(x1,y1);
            let left_point_on_graph = field_point_on_curve::new_field_point_on_graph(left_point,g1)
                .unwrap();

            let right_point = field_point::new_point(x2,y2);
            let right_point_on_graph = field_point_on_curve::new_field_point_on_graph(right_point,g2)
                .unwrap();

            let sum = left_point_on_graph+right_point_on_graph;
            println!("F: {} 楕円曲線y^2 = x^3 + 7 上での 点{} + {} = {}",field,left_point,right_point,sum.point);
        }


    }
    println!("Hello, world!");
}
