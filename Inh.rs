// Define a trait that acts like a base class
trait Animal {
    fn speak(&self); // Each animal will define its own way of speaking
    fn walk(&self) {
        // Default behavior for walking
        println!("Walking on four legs...");
    }
}

// Implement the Animal trait for a specific struct
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }
}

// Another struct that implements the Animal trait
struct Cat {
    name: String,
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} says: Meow!", self.name);
    }
}

// Another trait that builds upon Animal (simulating inheritance)
trait Pet: Animal {
    fn owner_name(&self) -> &str; // Pets have owners
}

// Implement the Pet trait for Dog
impl Pet for Dog {
    fn owner_name(&self) -> &str {
        "John" // Example owner
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Buddy"),
    };
    let cat = Cat {
        name: String::from("Whiskers"),
    };

    // Call trait methods
    dog.speak();
    dog.walk();
    println!("Owner of {}: {}", dog.name, dog.owner_name());

    cat.speak();
    cat.walk();
}
