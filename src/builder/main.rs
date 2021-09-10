#[derive(Debug, Copy, Clone)]
enum Gender {
    Male,
    Female
}
#[derive(Debug)]
struct Person {
    name: String,
    gender: Gender,
    age: u8
}

#[derive(Debug)]
struct PersonBuilder {
    name: String,
    gender: Gender,
    age: u8
}

impl Default for PersonBuilder {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            gender: Gender::Male,
            age: 0
        }
    }
}

impl PersonBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.name = name.as_ref().to_owned();
        self
    }

    fn get_gender(&self) -> Gender {
        self.gender
    }

    fn set_gender(&mut self, gender: Gender) -> &mut Self {
        self.gender = gender;
        self
    }

    fn get_age(&self) -> u8 {
        self.age
    }

    fn set_age(&mut self, age: u8) -> &mut Self {
        self.age = age;
        self
    }

    fn finish(&mut self)  -> Person {
        Person {
            name: self.name.clone(),
            age: self.age,
            gender: self.gender
        }
    }
}


fn main() {
    let mut builder = PersonBuilder::new();
    builder.set_name("Bob");
    builder.set_age(12);
    builder.set_gender(Gender::Male);
    println!("{:?}", builder.finish());

    builder.set_name("Mary");
    builder.set_gender(Gender::Female);
    println!("{:?}", builder.finish());
}
