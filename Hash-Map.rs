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

//fn three
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    // Inserting values
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    scores.insert("red", 30);
    // Accessing values
    if let Some(score) = scores.get("Blue") {
        println!("Blue team score: {}", score);
    }
    // Accessing values
    if let Some(score) = scores.get("red") {
        println!("red team score: {}", score);
    }
    // Updating a value
    if let Some(score) = scores.get_mut("Blue") {
        *score += 10;
    }
    // Checking for existence
    if scores.contains_key("Yellow") {
        println!("Yellow team is present");
    }
    // Removing a value
    scores.remove("Yellow");
    // Iterating over entries
    for (team, score) in &scores {
        println!("{}: {}", team, score);
    }
    // Getting the length
    println!("Number of teams: {}", scores.len());
    // Clearing the HashMap
    scores.clear();
}

