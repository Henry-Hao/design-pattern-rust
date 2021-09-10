
trait Prototype: Sized {
    fn set_color(&mut self, color: ColorType);
    fn set_shape(&mut self, shape: ShapeType);
}

#[derive(Debug, Clone)]
struct Shape {
    color: ColorType,
    shape: ShapeType
}

#[derive(Debug, Clone)]
enum ColorType {
    Red,
    Yellow,
    Blue
}

#[derive(Debug, Clone)]
enum ShapeType {
    Circle,
    Square,
    Rectangle
}

impl Shape {
    fn new() -> Self {
        Shape {
            color: ColorType::Red,
            shape: ShapeType::Circle
        }
    }
}

impl Prototype for Shape {
    fn set_color(&mut self, color: ColorType) {
        self.color = color;
    }

    fn set_shape(&mut self, shape: ShapeType) {
        self.shape = shape;
    }
}

fn main() {
    let a = Shape::new();
    println!("{:?}", a);
    let mut b = a.clone();
    b.set_shape(ShapeType::Rectangle);
    b.set_color(ColorType::Yellow);
    println!("{:?}", b);
}
