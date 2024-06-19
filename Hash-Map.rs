// Proper 
use std::collections::HashMap;
fn main(){
     let mut accountInfo = HashMap::new();
     accountInfo.insert("Boby","Overdraft!");
     accountInfo.insert("Ravi","Good Standing!");
     accountInfo.insert("Robby","Insufficient funds!");
     accountInfo.insert("Raviya","Overpowered!");
  println!("The size of the map is {}", accountInfo.len());
}
