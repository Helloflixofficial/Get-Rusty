// I'm trying to recall where I left learning this HashMap.
use std::collections::HashMap;
fn main(){
     let mut accountinfo = HashMap::new();
     accountinfo.insert("Boby","Overdraft!");
     accountinfo.insert("Ravi","Good Standing!");
     accountinfo.insert("Robby","Insufficient funds!");
     accountinfo.insert("Raviya","Overpowered!");
  println!("The size of the map is {}", accountinfo.len());
  let mut bardrinks =  HashMap:: new();
  bardrinks.insert("vodka",true);
  bardrinks.insert("beer",false);
  bardrinks.insert("whiskey",true);
  println!("THE size of thebardrinks is {}",bardrinks.len()); //before 
  bardrinks.remove(&"whiskey");
  println!("THE size of thebardrinks is {}",bardrinks.len());//after
}


