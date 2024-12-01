use std::ops::Deref;

// Demonstrating `Sized` types
fn print_sized_value<T: Sized>(value: T) {
    println!("Sized value: {:?}", value);
}

// Demonstrating `Unsize` with trait objects
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

fn main() {
    // Sized example
    let int_value: i32 = 42;
    let float_value: f64 = 3.14;
    print_sized_value(int_value);
    print_sized_value(float_value);

    // Unsize example
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 7.0,
    };

    // Using references to unsized trait objects
    print_area(&circle);
    print_area(&rectangle);

    // Demonstrating Box<T> for unsized types
    let boxed_circle: Box<dyn Shape> = Box::new(Circle { radius: 10.0 });
    print_area(&*boxed_circle);

    // Demonstrating slices (unsized types)
    let array = [1, 2, 3, 4, 5];
    let slice: &[i32] = &array; // Slices are dynamically sized
    println!("Slice: {:?}", slice);
}
