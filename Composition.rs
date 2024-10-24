struct Person {
    name: String,
    age: u32,
}

struct Employee {
    person: Person,
    job_title: String,
}

impl Employee {
    fn get_info(&self) -> String {
        format!("{} is an {} and is {} years old.", self.person.name, self.job_title, self.person.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    let employee = Employee {
        person,
        job_title: String::from("Engineer"),
    };
    
    println!("{}", employee.get_info());
}
