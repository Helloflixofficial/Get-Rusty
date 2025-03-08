use serde::{Serialize,Deserialize}
use axum::{routing::get, Router};
use std::sync::{Arc, Mutex};
#[tokio::main]

#[tokio::main]
async fn main() {
    let list = Arc::new(Mutex::new(PersonList {
      list: vec![]
    }));
    let app = app(list);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn app(list: Arc<Mutex<PersonList>>) -> Router {
  Router::new().route("/", get(|| async {"hello world!"})).route("/list", get(get_list)).route("/person/{id}", get(get_single_person)).route("/add_person", post(add_new_person)).route("/remove_person/{id}", delete(remove_person)).route("/update_person/{id}", put(update_person)).with_state(list)
}

async fn get_list(State(list): State<Arc<Mutex<PersonList>>>) -> impl IntoResponse {
    let person_list = list.lock().unwrap().clone();
    let json_data = serde_json::to_string_pretty(&person_list).unwrap();
    (StatusCode::OK, json_data).into_response()
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
