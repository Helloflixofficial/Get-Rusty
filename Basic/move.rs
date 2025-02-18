fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];
    let result = sum(vec1);
    println!("Sum of elements: {}", result);
}

fn sum(v: Vec<i32>) -> i32 {
    v.iter().sum() 
}
