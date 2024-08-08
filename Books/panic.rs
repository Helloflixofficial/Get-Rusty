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



use std::fs::File;

 fn main() {
 
     // panic!("This will cause the program to abruptly end");
     
     // let f = File::open("doesnotexist.txt").expect("No such thing!");
     
     let solution = is_seven(8).unwrap();
     println!("the solution is {}", solution);
     println!("The progam is working here");

 } 
 
 fn is_seven(n:i32) -> Result <bool,String> {
     if n == 7 {
        return Ok(true);
     } else {
        return Err("This is not seven".to_string());
     }
 }
