trait Strategy {
    fn operate(&self, a: i32, b: i32) -> i32;
}

struct OperationAdd;
struct OperationSub;
struct OperationMul;

impl Strategy for OperationAdd {
    fn operate(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
impl Strategy for OperationSub {
    fn operate(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}
impl Strategy for OperationMul {
    fn operate(&self, a: i32, b: i32) -> i32 {
        a * b
    }
}

struct Context {
    oprand_a: i32,
    oprand_b: i32,
    strategy: Box<dyn Strategy>,
}

impl Context {
    fn operate(&self) {
        println!("{}", self.strategy.operate(self.oprand_a, self.oprand_b));
    }
}

fn main() {
    let mut context = Context {
        oprand_a: 4,
        oprand_b: 3,
        strategy: Box::new(OperationAdd),
    };
    context.operate();
    context.strategy = Box::new(OperationMul);
    context.operate();
    context.strategy = Box::new(OperationSub);
    context.operate();
}
