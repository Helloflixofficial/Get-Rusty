//basic's
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);
}

fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);
}

fn main() {
    let mut v = vec![1, 2, 3, 4];
    v.pop(); // removes the last element
    println!("{:?}", v);

    v.remove(1); // removes the element at index 1
    println!("{:?}", v);
}

fn main(){
    let mut v = vec![1,2,3,4,5];
    for i in &mut v {
        *i += 10;
    }
    println!("{:?}",v);
}

fn main(){
    let v = vec![1,2,3,4,5];
    for i in v {
        println!("{:?}",i)
    }
}


fn main(){
    let v = vec![1,2,3,4,5];
    println!("{:?}",v);
}

fn main() {
    let v = vec![1,2,3,4,5];
    match v.get(3){
        Some(third) => println!("the third elements is {}", third),
        None => println!("there is no third elemet"),
    }
}

fn main(){
    let v = vec![1,2,3,4,5];
    let slice = &v[1..3];
    println!("{:?}",slice)
}

//daily cota
fn main(){
    let mut num = Vec::new();
   num.push(10);
   num.push(30);
   num.push(60);
   println!("initial data {:?}",num);

   if let Some(first) = num.get(0){
    println!("First element : {}", first);
   }

   for numb in &num{
    println!("Number : {}",numb);
   }

   for numb in &mut num {
    *numb += 5;
   }

   println!("Modify vector :  {:?}",num);

   if num.len()> 1 {
    num.remove(1);
   }
   println!("After  removing second element : {:?}",num);
   let slice =&num[..];
   println!("slice vector : {:? }", slice)
}



fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);
    if let Some(first) = numbers.get_mut(0) {
        *first = 10;
    }

    println!("Elements in the vector:");
    for number in &numbers {
        println!("{}", number);
    }

    println!("Elements in the vector using indices:");
    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }
}

