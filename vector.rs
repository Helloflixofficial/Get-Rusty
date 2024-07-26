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
