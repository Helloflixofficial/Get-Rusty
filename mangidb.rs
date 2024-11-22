use mongodb::{options::ClientOptions, Client};
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let uri = "mongodb://localhost:27017";
    let options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(options)?;
    let db = client.database("test_db");
    let collection = db.collection::<mongodb::bson::Document>("test_collection");
    let databases = client.list_database_names(None, None).await?;
    println!("Connected to MongoDB! Available databases: {:?}", databases);
    Ok(())
}
