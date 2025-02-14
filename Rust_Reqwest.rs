use serde::Deserialize;

#[tokio::main]
async fn main() {
    let image_response: ImageReponse = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await.unwrap().json::<ImageReponse>().await.unwrap();

    println!("{:?}", image_response);
}

#[derive(Deserialize, Debug)]
struct ImageReponse {
    message: String,
    status: String,
}
