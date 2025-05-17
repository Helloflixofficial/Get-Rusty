use reqwest;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    println!("🔍 Fetching anime data...");

    let url = "https://consumet-api-rz8l.onrender.com/meta/anilist/naruto";
    let response = reqwest::get(url).await;

    match response {
        Ok(resp) => {
            let json = resp.json::<ApiResponse>().await;

            match json {
                Ok(data) => {
                    for (i, anime) in data.results.iter().enumerate().take(5) {
                        let title = anime
                            .title
                            .english
                            .as_deref()
                            .or(anime.title.romaji.as_deref())
                            .or(anime.title.native.as_deref())
                            .unwrap_or("Unknown Title");

                        println!("\n#{} 🎬 Title: {}", i + 1, title);
                        println!("🖼️ Image: {}", anime.image);
                    }
                }
                Err(e) => eprintln!("❌ Failed to parse JSON: {}", e),
            }
        }
        Err(e) => eprintln!("❌ Failed to fetch data: {}", e),
    }
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    results: Vec<Anime>,
}

#[derive(Debug, Deserialize)]
struct Anime {
    #[allow(dead_code)]
    id: String,
    title: Title,
    image: String,
}

#[derive(Debug, Deserialize)]
struct Title {
    romaji: Option<String>,
    english: Option<String>,
    native: Option<String>,
}
