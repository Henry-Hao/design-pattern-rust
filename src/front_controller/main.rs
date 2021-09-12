struct FrontController {
    dispatcher: Dispatcher
}

impl FrontController {
    fn new(dispatcher: Dispatcher) -> Self {
        Self { dispatcher }
    }

    fn is_authenticated(&self) -> bool {
        println!("Is user authenticated?");
        true
    }

    fn track_request(&self, path: &dyn AsRef<str>) {
        println!("Requested: {}", path.as_ref());
    }

    fn dispatch_request(&self, path: &dyn AsRef<str>) {
        self.track_request(path);
        if self.is_authenticated() {
            self.dispatcher.dispatch(path);
        }
    }
}

struct Dispatcher {
    student_view: StudentView,
    home_view: HomeView
}

impl Dispatcher {
    fn new(student_view: StudentView, home_view: HomeView) -> Self {
        Self {
            student_view,
            home_view
        }
    }

    fn dispatch(&self, path: &dyn AsRef<str>) {
        if path.as_ref().eq_ignore_ascii_case("student") {
            self.student_view.show();
        } else {
            self.home_view.show();
        }
    }
}



struct HomeView;
struct StudentView;

impl HomeView {
    fn show(&self) {
        println!("HomeView show");
    }
}

impl StudentView {
    fn show(&self) {
        println!("StudentView show");
    }
}
fn main() {
    let dispatcher = Dispatcher::new(StudentView, HomeView);
    let controller = FrontController::new(dispatcher);
    controller.dispatch_request(&"student");
    controller.dispatch_request(&"Home");
    controller.dispatch_request(&"STudent");
}
