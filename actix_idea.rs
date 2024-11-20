use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct InputData {
    name: String,
    age: u32,
}

#[derive(Serialize)]
struct ResponseData {
    message: String,
}

async fn greet_user(input: web::Json<InputData>) -> impl Responder {
    let response = ResponseData {
        message: format!("Hello, {}! You are {} years old.", input.name, input.age),
    };
    web::Json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/greet", web::post().to(greet_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
