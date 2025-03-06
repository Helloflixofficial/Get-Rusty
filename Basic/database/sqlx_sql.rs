use sqlx::{mysql, prelude::FromRow, Connection};

#[tokio::main]
async fn main() {
    let opt = mysql::MySqlConnectOptions::new().host("localhost").port(3306).username("root").password("123456").database("test");
    let mut connection = mysql::MySqlConnection::connect_with(&opt).await.unwrap();
    sqlx::query("INSERT INTO users (name) VALUES(?)").bind(&"jim").execute(&mut connection).await.unwrap();
    sqlx::query("INSERT INTO users (name) VALUES(?)").bind(&"pam").execute(&mut connection).await.unwrap();
    let rows: Vec<Person> = sqlx::query_as("SELECT * FROM users").fetch_all(&mut connection).await.unwrap();

    for row in rows  {
        println!("{:?}", row);
    }

}

#[derive(Debug,FromRow)]
struct Person {
    id: i32,
    name: String,
}
