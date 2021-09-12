trait Shape {
    fn draw(&self);
}

trait Color {
    fn fill(&self);
}

enum ShapeType {
    Circle,
    Square,
    Rectangle,
}

struct Circle;
struct Square;
struct Rectangle;

impl Shape for Circle {
    fn draw(&self) {
        println!("Draw Circle");
    }
}

impl Shape for Square {
    fn draw(&self) {
        println!("Draw Square");
    }
}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Draw Rectangle");
    }
}

enum ColorType {
    Red,
    Blue,
    Yellow,
}

struct Red;
struct Blue;
struct Yellow;

impl Color for Red {
    fn fill(&self) {
        println!("Fill Red");
    }
}

impl Color for Blue {
    fn fill(&self) {
        println!("Fill Blue");
    }
}

impl Color for Yellow {
    fn fill(&self) {
        println!("Fill Yellow");
    }
}

enum FactoryType {
    ShapeFactory,
    ColorFactory,
}
struct ShapeFactory;
struct ColorFactory;

trait Factory {
    fn create_shape(&self, t: ShapeType) -> Option<Box<dyn Shape>>;
    fn create_color(&self, t: ColorType) -> Option<Box<dyn Color>>;
}
impl Factory for ShapeFactory {
    fn create_shape(&self, t: ShapeType) -> Option<Box<dyn Shape>> {
        Some(match t {
            ShapeType::Circle => Box::new(Circle),
            ShapeType::Square => Box::new(Square),
            ShapeType::Rectangle => Box::new(Rectangle),
        })
    }

    fn create_color(&self, _t: ColorType) -> Option<Box<dyn Color>> {
        None
    }
}

impl Factory for ColorFactory {
    fn create_color(&self, t: ColorType) -> Option<Box<dyn Color>> {
        Some(match t {
            ColorType::Red => Box::new(Red),
            ColorType::Yellow => Box::new(Yellow),
            ColorType::Blue => Box::new(Blue),
        })
    }

    fn create_shape(&self, _t: ShapeType) -> Option<Box<dyn Shape>> {
        None
    }
}

trait AbstractFactory {
    fn create_factory(t: FactoryType) -> Box<dyn Factory>;
}

struct F;
impl AbstractFactory for F {
    fn create_factory(t: FactoryType) -> Box<dyn Factory> {
        match t {
            FactoryType::ShapeFactory => Box::new(ShapeFactory),
            FactoryType::ColorFactory => Box::new(ColorFactory),
        }
    }
}

fn main() {
    let f = F::create_factory(FactoryType::ShapeFactory);
    f.create_shape(ShapeType::Circle).unwrap().draw();
    f.create_shape(ShapeType::Square).unwrap().draw();
    f.create_shape(ShapeType::Rectangle).unwrap().draw();
    let f = F::create_factory(FactoryType::ColorFactory);
    f.create_color(ColorType::Red).unwrap().fill();
    f.create_color(ColorType::Yellow).unwrap().fill();
    f.create_color(ColorType::Blue).unwrap().fill();
}
