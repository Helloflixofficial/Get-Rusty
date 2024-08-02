//easy one
fn main(){
    let data = seven(10).unwrap();
    println!("YOU HVAE ENTER THE NUMBER : {}",data);

}

fn seven(n:i32) -> Result <bool, String> {
    if n == 7 {
        return Ok(true);
    }else{
        return Err("This is not  a seven".to_string());
    }
}
