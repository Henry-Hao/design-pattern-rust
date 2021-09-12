use std::fmt::Debug;

trait ComputerPart: Debug {
    fn accept(&self, visitor: &Box<&dyn ComputerPartVisitor>);
}
#[derive(Debug)]
struct Keyboard;
#[derive(Debug)]
struct Mouse;
#[derive(Debug)]
struct Monitor;
#[derive(Debug)]
struct Computer {
    keyboard: Keyboard,
    mouse: Mouse,
    monitor: Monitor,
}

impl ComputerPart for Computer {
    fn accept(&self, visitor: &Box<&dyn ComputerPartVisitor>) {
        self.keyboard.accept(visitor);
        self.mouse.accept(visitor);
        self.monitor.accept(visitor);
    }
}

impl Computer {
    fn new() -> Self {
        Self {
            keyboard: Keyboard,
            mouse: Mouse,
            monitor: Monitor,
        }
    }
}

impl ComputerPart for Mouse {
    fn accept(&self, visitor: &Box<&dyn ComputerPartVisitor>) {
        visitor.visit(&Box::new(self as &dyn ComputerPart));
    }
}

impl ComputerPart for Monitor {
    fn accept(&self, visitor: &Box<&dyn ComputerPartVisitor>) {
        visitor.visit(&Box::new(self as &dyn ComputerPart));
    }
}

impl ComputerPart for Keyboard {
    fn accept(&self, visitor: &Box<&dyn ComputerPartVisitor>) {
        visitor.visit(&Box::new(self as &dyn ComputerPart));
    }
}

trait ComputerPartVisitor {
    fn visit(&self, part: &Box<&dyn ComputerPart>);
}

struct ComputerPartDisplayVisitor;
impl ComputerPartVisitor for ComputerPartDisplayVisitor {
    fn visit(&self, part: &Box<&dyn ComputerPart>) {
        println!("{:?} is visiting", part);
    }
}

fn main() {
    let computer = Computer::new();
    computer.accept(&Box::new(&ComputerPartDisplayVisitor));
}
