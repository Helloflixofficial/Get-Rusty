use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

// Define a simple struct to represent a greeting
#[derive(Serialize, Deserialize)]
struct Greeting {
    message: String,
}

// Handler function for the /greet endpoint
async fn greet() -> impl Responder {
    // Create a Greeting instance
    let greeting = Greeting {
        message: String::from("Hello, Rusty World!"),
    };

    // Return a JSON response with the greeting
    HttpResponse::Ok().json(greeting)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server
    HttpServer::new(|| {
        // Create an Actix web application
        App::new()
            // Define the /greet endpoint with the greet handler
            .route("/greet", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")? // Bind to the localhost on port 8080
    .run()
    .await
}
