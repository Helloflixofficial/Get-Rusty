use serde::{Serialize,Deserialize}
use axum::{routing::get, Router};
use std::sync::{Arc, Mutex};
#[tokio::main]

async fn main() {}


fn app(list: Arc<Mutex<PersonList>>) -> Router {
    Router::new().route("/", get(|| async {"hello world!"})).route("/list", get(get_list)).route("/person/{id}", get(get_single_person)).route("/add_person", post(add_new_person)).route("/remove_person/{id}", delete(remove_person)).route("/update_person/{id}", put(update_person)).with_state(list)
  }



#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Person {
    id: i32,
    name: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PersonList {
    list:Vec<Person>
}


#[derive(Deserialize)]
struct PersonRequest{
    name:String
}
