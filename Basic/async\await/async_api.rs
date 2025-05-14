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
