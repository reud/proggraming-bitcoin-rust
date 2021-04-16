mod field_point;
mod field_element;
mod point;
mod field_graph;

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
    println!("Hello, world!");
}
