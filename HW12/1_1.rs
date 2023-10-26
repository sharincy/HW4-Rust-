use std::f64::consts::PI;


#[derive(Debug, Clone)]
enum Shape {
    Circle(i64, i64, i64),
    Rectangle(i64, i64, i64, i64),
    Triangle((f64, f64), (f64, f64), (f64, f64)),
}

trait Shapefn {
    fn rep_string(&self) -> String;
    fn area(&self) -> f64;
}

impl Shapefn for Shape {
    fn rep_string(&self) -> String {
        match self {
            Self::Circle(x, y, r) => format!("<Circle: {}, {}, {}>", x, y, r),
            Self::Rectangle(x, y, w, h) => format!("<Rectangle: {}, {}, {}, {}>", x, y, w, h),
            Self::Triangle(a, b, c) => format!("<Triangle: p1{:?}, p2{:?}, p3{:?}>", a, b, c),
        }
    }

    fn area(&self) -> f64 {
        match self {
            Self::Circle(_, _, r) => {
                let are = PI * (*r as f64) * (*r as f64);
                are
            }
            Self::Rectangle(_, _, w, h) => {
                let are = (*w as f64) * (*h as f64);
                are
            }
            Self::Triangle((x1, y1), (x2, y2), (x3, y3)) => {
                let are = 0.5 * ((x1 - x3) * (y2 - y1) - (x1 - x2) * (y3 - y1));
                are.abs()  
            }
        }
    }
}

const INPUT_SHAPES: &[Shape] = &[
    Shape::Circle(0, 0, 1),
    Shape::Circle(50, 50, 15),
    Shape::Rectangle(40, 40, 20, 20),
    Shape::Rectangle(10, 40, 15, 10),
];

const EXPECTED: &[&str] = &[
    "<Circle: 0, 0, 1>, area: 3.14",
    "<Circle: 50, 50, 15>, area: 706.86",
    "<Rectangle: 40, 40, 20, 20>, area: 400.00",
    "<Rectangle: 10, 40, 15, 10>, area: 150.00",
];

fn main() {
    let shape_list = INPUT_SHAPES.to_vec();
    let output: Vec<_> = shape_list
        .iter()
        .map(|s| format!("{}, area: {:.2}", s.rep_string(), s.area()))
        .collect();

    for (output_str, expected_str) in output.iter().zip(EXPECTED.iter()) {
        assert_eq!(output_str, expected_str);
    }
}
