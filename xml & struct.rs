use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};
fn main() {
    let xml = r#"<Person><name>BOBY<name/><age>24<age/><Person/>"#;

    let person: Person = from_str(xml).unwrap();
    println!("{:?}", person);

    let person_data = to_string(&person).unwrap();
    println!("{}", person_data);
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
}
