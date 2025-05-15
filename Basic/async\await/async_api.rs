use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    location: Location,
    current: Current,
}

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    country: String,
}

#[derive(Debug, Deserialize)]
struct Current {
    temperature: f64,
    weather_descriptions: Vec<String>,
}

async fn fetch_weather(city: &str) -> Result<WeatherResponse, Error> {
    let api_url = format!(
        "http://api.weatherstack.com/current?access_key=YOUR_API_KEY&query={}",
        city
    );

    let response = reqwest::get(&api_url)
        .await?
        .json::<WeatherResponse>()
        .await?;

    Ok(response)
}

#[tokio::main]
async fn main() {
    let city = "London";

    match fetch_weather(city).await {
        Ok(weather) => {
            println!(
                "Weather in {}, {}: {}°C, {}",
                weather.location.name,
                weather.location.country,
                weather.current.temperature,
                weather.current.weather_descriptions.join(", ")
            );
        }
        Err(e) => eprintln!("Error fetching weather: {}", e),
    }
}












use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Joke {
    setup: String,
    punchline: String,
}

#[tokio::main]
async fn main() {
    println!("🟡 Getting a joke...");

    match get_joke().await {
        Ok(joke) => {
            println!("\n🤣 Here's a random joke:");
            println!("👉 {}", joke.setup);
            println!("😆 {}", joke.punchline);
        }
        Err(e) => {
            println!("❌ Failed to get a joke: {}", e);
        }
    }
}

async fn get_joke() -> Result<Joke, Box<dyn std::error::Error>> {
    let url = "https://official-joke-api.appspot.com/random_joke";
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    let joke = response.json::<Joke>().await?;
    Ok(joke)
}

tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }


