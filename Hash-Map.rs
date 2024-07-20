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





//////////
//Cargo.toml
[package]
name = "rustdata"
version = "0.1.0"
edition = "2021"
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
//
//main.rs
use std::collections::HashMap;
use std::error::Error;
use serde_json::Value;
use reqwest;

async fn fetch_json(url: &str) -> Result<Value, Box<dyn Error>> {
    let response = reqwest::get(url).await?.text().await?;
    let json: Value = serde_json::from_str(&response)?;
    Ok(json)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut list: HashMap<String, f32> = HashMap::new();
    list.insert("NVA".to_string(), 200.50);
    list.insert("APP".to_string(), 225.25);
    list.insert("remove".to_string(), 111.25);
    println!("The contents of the hashmap are: {:#?}", list);
    list.remove(&"remove".to_string());
    list.insert("remove".to_string(), 222.25);
    println!("The size of the hashmap is: {}", list.len());
    println!("The contents of the hashmap are: {:#?}", list); 
    let url = "https://webdis-7ies.onrender.com/recent-release";
    let data = fetch_json(url).await?;
    println!("{:#?}", data);
    Ok(())
}


pub fn main(){
    let mut data: HashMap<String,f32> = HashMap::new();
    data.insert("Sharmaji".to_string(), 007.3);
    data.insert("Sharmaji".to_string(), 003.3);
    data.entry("Kumar".to_string()).or_insert(55.2);
    println!("{:#?}", data);
}



fn main() {
    let mut nums = vec![1, 2, 3];
    let sum: i32 = nums.iter().sum();
    println!("Sum: {}", sum);
    for n in &nums {
        println!("{}", n);
    }
    
    nums.insert(1, 100);
    if let Some(pos) = nums.iter().position(|&x| x == 100) {
        nums.remove(pos);
    }
    
    let contains = nums.contains(&100);
    println!("Contains 100: {}", contains);
    
 
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::from(nums.clone());
    let mut min_heap: Vec<_> = heap.into_sorted_vec();
    min_heap.reverse();
    println!("Heap: {:?}", min_heap);
}

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 50);
    scores.insert(String::from("Boby"), 30);
    scores.insert(String::from("Carol"), 100);

    let alice_score = scores.get("Alice");

    match alice_score {
        Some(score) => println!("Alice's score is: {}", score),
        None => println!("Alice's score is not found"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Bob"), 40);
    scores.remove("Carol");
    println!("Final scores: {:?}", scores);
}
