use std::thread;

static mut INSTANCE: Option<Singleton> = None;

#[derive(Debug)]
struct Singleton {
    val: i32
}

impl Singleton {
    fn get_instance() ->  &'static mut Singleton{
        unsafe {
            match INSTANCE {
                Some(ref mut instance) => instance,
                None => {
                    INSTANCE = Some(Singleton{ val: 100 });
                    Singleton::get_instance()
                }
            }
        }
    }
}

fn main() {
    let a = Singleton::get_instance();
    let b = Singleton::get_instance();
    println!("{:?}", a);
    println!("{:?}", b);
    a.val = 10;
    println!("{:?}", a);
    thread::spawn(|| {
        println!("{:?}", Singleton::get_instance());
        Singleton::get_instance().val = 99;
    }).join().unwrap();
    println!("{:?}", a);

}
