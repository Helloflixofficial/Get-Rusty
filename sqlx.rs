use sqlx::{sqlite::SqlitePoolOptions, Row};
use tokio;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool to the SQLite database
    let db_url = "sqlite::memory:"; // In-memory database for testing
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    // Create a table
    sqlx::query(
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Insert data into the table
    sqlx::query("INSERT INTO users (name, age) VALUES (?, ?)")
        .bind("Alice")
        .bind(30)
        .execute(&pool)
        .await?;

    sqlx::query("INSERT INTO users (name, age) VALUES (?, ?)")
        .bind("Bob")
        .bind(25)
        .execute(&pool)
        .await?;

    // Query the data
    let rows = sqlx::query("SELECT id, name, age FROM users")
        .fetch_all(&pool)
        .await?;

    // Print the results
    for row in rows {
        let id: i64 = row.get("id");
        let name: String = row.get("name");
        let age: i32 = row.get("age");

        println!("User: id={}, name={}, age={}", id, name, age);
    }

    Ok(())
}
