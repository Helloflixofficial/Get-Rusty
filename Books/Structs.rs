
/*

Wbat are Structs in Rust

Structs are similar to tuples, discussed in “The Tuple Type” section, 
in that both hold multiple related values. Like tuples, the pieces of a struct can be different types. 
Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean. 
Adding these names means that structs are more flexible than tuples: 
you don’t have to rely on the order of the data to specify or access the values of an instance.

Structs use key value pairs to define data.

Rust allows to combine data items of different types, 
including other structures. 

*****

Methods are functions which belong in the scope within a structure.
You declare methods with the impl keyword outside of the structure block.
Important to note that the parameter of a method will be always self, 
which represents the calling instance of the structure. 

Example:

struct The_struct {

impl The_struct {
    
    fn method_name(&self) {
        
    }
}
    
}

*/

// Struct Example

struct House {

    door:String,
    garden:String,
    property_value:u32
    
} 

// Exercise Solution

struct Triangle {
    base:u32,
    height:u32 
}

impl Triangle {
    
    fn find_area(&self) -> u32 {
       return self.base * self.height / 2;
    } 
}



fn main() {
    
    let home1 = House {
        door:String::from("Blue"),
        garden:String::from("Beautiful"), 
        property_value:1000000
    }; 
    
    println!("door is: {}, garden is: {}, property value is: {}",
    home1.door, home1.garden, home1.property_value);
    
    let new_triangle = Triangle {
        base:10,
        height:30 
    };
    
    println!("the area of the triangle is: {}", new_triangle.find_area());
    
}

/*

Build A Triangle Calculator in Rust with Structs

1. Write a Triangle struct that takes the key pair values of base and height as unsigned integers
2. write a method for the Triangle which can calculate the area of the triangle (remember the triangle area
formula is base * height / 2)
3. Create a new triangle struct and set the base to 10 and the height to 30
4. Print the method calculation for the area of the new triange.

Good Geometric luck! 

*/


 
