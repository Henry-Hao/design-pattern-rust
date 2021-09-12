trait Observer {
    fn new(id: i32) -> Self;
    fn update(&self, data: u8);
}

trait Observable<'a, T: Observer> {
    fn attach(&mut self, observer: &'a T);
    fn detach(&mut self, observer: &'a T);
    fn notify(&self);
}

#[derive(PartialEq)]
struct BinaryObserver {
    id: i32,
}

impl Observer for BinaryObserver {
    fn update(&self, num: u8) {
        println!(
            "Update from BinaryObserver {}, binary of {} is: {:b}",
            self.id, num, num
        );
    }

    fn new(id: i32) -> Self {
        Self { id }
    }
}
//
// #[derive(PartialEq)]
// struct OctalObserver {
//     id: i32
// }
//
// impl Observer for OctalObserver {
//     fn update(&self, num: u8) {
//         println!("Update from OctalObserver {}, octal of {} is: {:o}", self.id, num, num);
//     }
//
//     fn new(id: i32) -> Self {
//         Self { id }
//     }
// }
//
// #[derive(PartialEq)]
// struct HexaObserver {
//     id: i32
// }
//
// impl Observer for HexaObserver {
//     fn update(&self, num: u8) {
//         println!("Update from HexaObserver {}, octal of {} is: {:x}", self.id, num, num);
//     }
//
//     fn new(id: i32) -> Self {
//         Self { id }
//     }
// }
//
struct Subject<'a, T: Observer> {
    data: u8,
    observers: Vec<&'a T>,
}

impl<'a, T: Observer + PartialEq> Observable<'a, T> for Subject<'a, T> {
    fn attach(&mut self, observer: &'a T) {
        self.observers.push(observer);
    }

    fn detach(&mut self, observer: &'a T) {
        if let Some(idx) = self.observers.iter().position(|&x| x == observer) {
            self.observers.remove(idx);
        }
    }

    fn notify(&self) {
        for observer in self.observers.iter() {
            observer.update(self.data);
        }
    }
}

impl<'a, T: Observer + PartialEq> Subject<'a, T> {
    fn new(data: u8) -> Subject<'a, T> {
        Self {
            data,
            observers: Vec::new(),
        }
    }

    fn set_data(&mut self, data: u8) {
        self.data = data;
        self.notify();
    }
}

fn main() {
    let mut subject = Subject::new(10);
    let observer_a = BinaryObserver::new(1);
    let observer_b = BinaryObserver::new(2);
    // TODO How to make the following line compile
    // let observer_b = OctalObserver::new(8);

    subject.attach(&observer_a);
    subject.attach(&observer_b);
    subject.notify();
    subject.set_data(16);
    subject.detach(&observer_a);
    subject.set_data(100);
}
