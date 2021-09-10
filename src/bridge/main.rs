trait DrawAPI {
    fn draw_circle(&self);
}

struct Shape {
    drawApi: Box<dyn DrawAPI>
}

struct Circle {
    x: u8,
    y: u8,
    radius: u8,
    shape: Shape
}

struct RedCircle;
struct GreenCircle;

impl DrawAPI for RedCircle {
    fn draw_circle(&self) {
        println!("Draw a red circle");
    }
}

impl DrawAPI for GreenCircle {
    fn draw_circle(&self) {
        println!("Draw a green circle");
    }
}

impl Circle {
    fn new(x: u8, y: u8, radius: u8, api: Box<dyn DrawAPI>) -> Self {
        Self {
            shape: Shape {
                drawApi: api
            },
            x,
            y,
            radius
        }
    }

    fn draw(&self) {
        println!("x:{}, y:{}, radius: {}", self.x, self.y, self.radius);
        self.shape.draw()
    }
}

impl Shape {
    fn draw(&self) {
        self.drawApi.draw_circle()
    }
}

fn main() {
    let c1 = Circle::new(1, 2, 3, Box::new(RedCircle));
    c1.draw();
    let c1 = Circle::new(4, 5, 6, Box::new(GreenCircle));
    c1.draw();
}
