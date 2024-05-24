What Are Enums in Rust

An enum type is a special data type that enables for a variable to be a set of predefined constants. 
The variable must be equal to one of the values that have been predefined for it. 
Common examples include compass directions (values of NORTH, SOUTH, EAST, and WEST) and the days of the week.

*/

// Example Enum

// the derive attribute makes the enum printable 

#[derive(Debug)]
enum TemperatureGrade {
    Hot,Cold,Medium
}

#[derive(Debug)]
struct City {
    name:String,
    temperature:TemperatureGrade 
} 

enum Shoes {
    Nike,
    Loafer,
    Vans
}

fn print_shoes(shoe:Shoes) {
    match shoe {
        Shoes::Nike => {
            println!("Great for running!");
        }, 
        Shoes::Loafer => {
            println!("Great for loafing around!");
        }, 
        Shoes::Vans => {
            println!("Great for skateboarding!");
        } 
    }
}

fn main() {
    
    let c1 = City {
        name:String::from("Alaska"),
        temperature:TemperatureGrade::Cold
    };
    
    let c2 = City {
        name:String::from("Miami"),
        temperature:TemperatureGrade::Hot
    };
    
    println!("{:?}",c1);
    println!("{:?}",c2);
    
    // Exercise Solution
    print_shoes(Shoes::Nike);
    print_shoes(Shoes::Loafer);
    print_shoes(Shoes::Vans);
    
}
