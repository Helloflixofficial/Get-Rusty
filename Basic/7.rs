
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

/*

Extra hint:
Match keyword -  
Match a single value
        1 => println!("One!"),
