#[derive(Debug)]
struct StudentModel {
    id: i32,
    name: String,
}

trait Controller {
    fn update_view(&self);
}

struct StudentView;

impl StudentView {
    fn print_student_detailes(&self, id: i32, name: &str) {
        println!("ID: {}, name: {}", id, name);
    }
}

struct StudentController {
    model: StudentModel,
    view: StudentView,
}

impl StudentModel {
    fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_id(&self) -> i32 {
        self.id
    }
}

impl StudentController {
    fn new(model: StudentModel, view: StudentView) -> Self {
        Self { model, view }
    }

    fn get_student_name(&self) -> &str {
        self.model.get_name()
    }

    fn set_student_name(&mut self, name: String) {
        self.model.set_name(name);
    }

    fn get_student_id(&self) -> i32 {
        self.model.get_id()
    }
}

impl Controller for StudentController {
    fn update_view(&self) {
        self.view
            .print_student_detailes(
                self.get_student_id(),
                self.get_student_name()
                )
    }
}

fn main() {
    let model = StudentModel::new(1, "Henry".to_owned());
    let view = StudentView;

    let mut controller = StudentController::new(model, view);
    controller.update_view();
    controller.set_student_name("Mary".to_owned());
    controller.update_view();
}
