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


// YO YO Leaning from GPT  So much confusion between syntex's But still..
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{Book, NewBook};

pub async fn get_books(pool: web::Data<PgPool>) -> impl Responder {
    let books = sqlx::query_as::<_, Book>("SELECT * FROM books")
        .fetch_all(pool.get_ref())
        .await;

    match books {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching books"),
    }
}

pub async fn create_book(
    pool: web::Data<PgPool>,
    new_book: web::Json<NewBook>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO books (title, author, published_date) VALUES ($1, $2, $3) RETURNING id",
        new_book.title,
        new_book.author,
        new_book.published_date,
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(record.id),
        Err(_) => HttpResponse::InternalServerError().body("Error adding book"),
    }
}

