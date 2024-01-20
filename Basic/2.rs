// Mutability
let mut x = 5;
x = 6;


// Shadowing
let x = 5;
let x = x * 2;


// Type alias
// `NanoSecond` is a new name for `u64`.
type NanoSecond = u64;



// Control Flow
if and if let
let num = Some(22);
if num.is_some() {
println!("number is: {}", num.unwrap());
}
// match pattern and assign variable
if let Some(i) = num {
println!("number is: {}", i);
}


// loop
let mut count = 0;
loop {
count += 1;
if count == 5 {
break; // Exit loop
}
}


// Nested loops & labels
'outer: loop {
'inner: loop {
// This breaks the inner loop
break;
// This breaks the outer loop
break 'outer;
}
}

// Returning from loops
let mut counter = 0;
let result = loop {
counter += 1;
if counter == 10 {
break counter;
}
};


// while and while let
while n < 101 {
n += 1;
}
let mut optional = Some(0);
while let Some(i) = optional {
print!("{}", i);
}

// for loop
for n in 1..101 {
println!("{}", n);
}
let names = vec!["Bogdan", "Wallace"];
for name in names.iter() {
println!("{}", name);
}

// match
let optional = Some(0);
match optional {
Some(i) => println!("{}", i),
None => println!("No value.")
}

