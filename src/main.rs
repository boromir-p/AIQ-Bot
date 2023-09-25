use serde::Deserialize;
use serde_json;
use teloxide::{
    prelude::*,
    types::{KeyboardButton, KeyboardMarkup},
};

#[derive(Debug, Deserialize)]
struct AirQualityResponse {
    data: AirQualityData,
}

#[derive(Debug, Deserialize)]
struct AirQualityData {
    aqi: i32,
}

#[tokio::main]
async fn main() {
    let telegram_token = String::from("<telegram_token>");

    let bot = Bot::new(telegram_token);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if msg.text().unwrap() == "Check Air Quality" {
            let response = reqwest::get(
                "http://api.waqi.info/feed/<your_city>/?token=<waqi_token>",
            )
            .await?;

            let json_str = response.text().await?;
            let air_quality_response: Result<AirQualityResponse, serde_json::Error> =
                serde_json::from_str(&json_str);

            let aqi: i32 = match air_quality_response {
                Ok(response) => {
                    // Access the AQI value
                    response.data.aqi
                }
                Err(_) => {
                    eprintln!("Failed to deserialize JSON:");
                    -1
                }
            };
            let text_msg = match aqi {
                0..=50 => format!("The air quality in Sofia is (green) good.\nAIQ = {}", aqi),
                51..=100 => format!("The air quality in Sofia is (yellow) acceptable.\n AIQ = {}", aqi),
                101..=150 => format!("The air quality in Sofia is (orange) unhealthy for sensitive groups.\nAIQ = {}", aqi),
                151..=200 => format!("The air quality in Sofia is (red) unhealthy for the general public.\nAIQ = {}", aqi),
                201..=300 => format!("The air quality in Sofia is (purple) very unhealthy.\nAIQ = {}", aqi),
                301..=1000 => format!("The air quality in Sofia is (marson) hazardous.\nAIQ = {}", aqi),
                _ => format!("Could not fetch the data"),
            };
            let _ = bot.send_message(msg.chat.id, text_msg)
                    .reply_markup(make_keyboard())
                    .await;
        }
        Ok(())
    })
    .await;
}

fn make_keyboard() -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![]; // Change KeyboardMarkup to KeyboardButton

    let debian_versions = ["Check Air Quality"];

    // Create a vector of KeyboardButton instances
    let buttons: Vec<KeyboardButton> = debian_versions
        .iter()
        .map(|text| KeyboardButton::new(text.to_string()))
        .collect();

    // Add the vector of buttons to the keyboard
    keyboard.push(buttons);

    KeyboardMarkup::new(keyboard)
}
