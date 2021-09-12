trait Shape {
    fn draw(&self);
}

struct Circle;
struct Square;
struct Rectangle;

enum Type {
    Circle,
    Square,
    Rectangle,
}

impl Shape for Circle {
    fn draw(&self) {
        println!("draw circle");
    }
}
impl Shape for Square {
    fn draw(&self) {
        println!("draw square");
    }
}
impl Shape for Rectangle {
    fn draw(&self) {
        println!("draw rectangle");
    }
}

fn shape_factory(t: Type) -> Box<dyn Shape> {
    match t {
        Type::Circle => Box::new(Circle),
        Type::Square => Box::new(Square),
        Type::Rectangle => Box::new(Rectangle),
    }
}

fn main() {
    let shape = shape_factory(Type::Square);
    shape.draw();
    let shape = shape_factory(Type::Circle);
    shape.draw();
    let shape = shape_factory(Type::Rectangle);
    shape.draw();
}
