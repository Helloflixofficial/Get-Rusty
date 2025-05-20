// src/main.rs
use axum::{routing::get, Router, Json};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

async fn get_user() -> Json<User> {
    Json(User {
        id: 1,
        name: "BOBY".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/user", get(get_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("🚀 Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
