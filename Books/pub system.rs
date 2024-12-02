// main.rs
mod my_module {
 
    pub fn greet(name: &str) {
        println!("Hello, {}!", name);
    }


    pub const APP_NAME: &str = "Rusty App";

    fn private_function() {
        println!("This is a private function.");
    }

    pub mod nested_module {
        pub fn nested_greet() {
            println!("Greetings from the nested module!");
        }
    }
}

fn main() {

    use my_module::{greet, APP_NAME};
    use my_module::nested_module::nested_greet;


    greet("Alice");
    println!("Welcome to {}!", APP_NAME);
    nested_greet();

    // Uncommenting the following line will cause an error because private_function is not public
    // my_module::private_function();
}
