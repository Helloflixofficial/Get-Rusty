
fn main() {
    
    // in general, {} will be automatically replaced with any arguments
    // and stringified
    
    println!("{} years old", 36);
    
    // named arguments also work
    
    println!("{user1}{action}{user2}",
            user1="Mary Lewis",
            user2="Jon Wick",
            action=" jackichan");
            
    
// Exercise: Within the function main add a println macro which takes two arguments, 
//x and y and stringifies them where x equals "hello " and 
// y equals "my friend".

    println!("{x}, {y}",
    x="hello",
    y="my friend")
            
}
