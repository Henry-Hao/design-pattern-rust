trait Criteria {
    fn meet_criteria(&self, list: &Vec<Person>) -> Vec<Person>;
}

struct AndCriteria {
    criteria: Box<dyn Criteria>,
    other: Box<dyn Criteria>,
}

struct OrCriteria {
    criteria: Box<dyn Criteria>,
    other: Box<dyn Criteria>,
}

struct CriteriaFemale;
struct CriteriaMale;
struct CriteriaSingle;

impl Criteria for CriteriaFemale {
    fn meet_criteria(&self, list: &Vec<Person>) -> Vec<Person> {
        list.clone()
            .into_iter()
            .filter(|person| !person.male)
            .collect()
    }
}

impl Criteria for CriteriaMale {
    fn meet_criteria(&self, list: &Vec<Person>) -> Vec<Person> {
        list.clone()
            .into_iter()
            .filter(|person| person.male)
            .collect()
    }
}

impl Criteria for CriteriaSingle {
    fn meet_criteria(&self, list: &Vec<Person>) -> Vec<Person> {
        list.clone()
            .into_iter()
            .filter(|person| person.single)
            .collect()
    }
}

impl AndCriteria {
    fn new(criteria: Box<dyn Criteria>, other: Box<dyn Criteria>) -> Self {
        Self { criteria, other }
    }
}
impl Criteria for AndCriteria {
    fn meet_criteria(&self, list: &Vec<Person>) -> Vec<Person> {
        let p = self.criteria.meet_criteria(list);
        self.other.meet_criteria(&p)
    }
}

impl OrCriteria {
    fn new(criteria: Box<dyn Criteria>, other: Box<dyn Criteria>) -> Self {
        Self { criteria, other }
    }
}
impl Criteria for OrCriteria {
    fn meet_criteria(&self, list: &Vec<Person>) -> Vec<Person> {
        let mut p1 = self.criteria.meet_criteria(&list);
        let mut p2 = self.other.meet_criteria(&list);
        p1.append(&mut p2);
        p1.dedup();
        p1
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Person {
    name: String,
    male: bool,
    single: bool,
}

impl Person {
    fn new(name: &dyn AsRef<str>, male: bool, single: bool) -> Person {
        Person {
            name: name.as_ref().to_owned(),
            male,
            single,
        }
    }
}
fn main() {
    let list: Vec<Person> = vec![
        Person::new(&"Bob", true, true),
        Person::new(&"Mary", false, true),
        Person::new(&"Shirley", false, false),
        Person::new(&"Henry", true, false),
    ];

    println!("Male:{:#?}", CriteriaMale.meet_criteria(&list));
    println!("Femail:{:#?}", CriteriaFemale.meet_criteria(&list));
    println!("Single:{:#?}", CriteriaSingle.meet_criteria(&list));
    println!(
        "Male&Single:{:#?}",
        AndCriteria::new(Box::new(CriteriaMale), Box::new(CriteriaSingle)).meet_criteria(&list)
    );
    println!(
        "Male|Single:{:#?}",
        OrCriteria::new(Box::new(CriteriaMale), Box::new(CriteriaSingle)).meet_criteria(&list)
    );
}
