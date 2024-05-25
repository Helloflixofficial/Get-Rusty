#[derive(Debug)]
enum TempGrade {
    Hot,
    Cold,
    Medium,
}

#[derive(Debug)]
struct City {
    name: String,
    temp: TempGrade, // Fix: Changed `tem` to `temp`
}

fn main() {
    let c1 = City {
        name: String::from("Alaska"),
        temp: TempGrade::Cold,
    };

    let c2 = City {
        name: String::from("Goa"),
        temp: TempGrade::Medium,
    };

    println!("the weather for Today is : {:?}", c1);
    println!("the weather for Today is : {:?}", c2);
}
