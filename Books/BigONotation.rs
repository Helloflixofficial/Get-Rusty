fn data(n:i32) -> i32{
    let mut sum = 0;
    for i in  0..=n{
        sum += i;
    }
    sum
}
fn data2(n: i32) -> i32 {
    n * (n + 1) / 2
}
fn main(){
    let result = data(10);
    let result2 = data2(10);
    println!("the sum is {:#}",result);
    println!("the sum is {:#}",result2);
}
