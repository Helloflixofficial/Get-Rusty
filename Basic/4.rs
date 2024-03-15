// Pattern Matching
Basics
let x = 5;
match x {
// matching literals
1 => println!("one"),
// matching multiple patterns
2 | 3 => println!("two or three"),
// matching ranges
4..=9 => println!("within range"),
// matching named variables
x => println!("{}", x),
// default case (ignores value)
_ => println!("default Case")
}





// Destructuring
struct Point {
x: i32,
y: i32,
}
let p = Point { x: 0, y: 7 };
match p {
Point { x, y: 0 } => {
println!("{}" , x);
},
Point { x, y } => {
println!("{} {}" , x, y);
},
}
enum Shape {
Rectangle { width: i32, height: i32 },
Circle(i32),
}
let shape = Shape::Circle(10);
match shape {
Shape::Rectangle { x, y } => //...
Shape::Circle(radius) => //...
}
