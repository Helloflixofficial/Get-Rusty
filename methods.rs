use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware, Error};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Define a struct to hold application state
struct AppState {
    counter: Mutex<i32>,
}

// Define a struct for JSON request body
#[derive(Deserialize)]
struct Info {
    name: String,
}

// Define a struct for JSON response
#[derive(Serialize)]
struct Greeting {
    message: String,
}

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, welcome to Actix Web!")
}

async fn greet_with_name(web::Path(name): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

async fn increment_counter(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Counter value: {}", counter))
}

async fn greet_json(info: web::Json<Info>) -> Result<HttpResponse, Error> {
    let greeting = Greeting {
        message: format!("Hello, {}!", info.name),
    };
    Ok(HttpResponse::Ok().json(greeting))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Share state across handlers
            .wrap(middleware::Logger::default()) // Enable logging middleware
            .route("/", web::get().to(greet)) // Route for the root
            .route("/greet/{name}", web::get().to(greet_with_name)) // Route with a path parameter
            .route("/counter", web::get().to(increment_counter)) // Route to increment counter
            .route("/greet_json", web::post().to(greet_json)) // Route for JSON greeting
    })
    .bind("127.0.0.1:8080")? // Bind to localhost on port 8080
    .run()
    .await
}
