use std::env;
use std::error::Error;
use std::time::Duration;

use busylib::model::assets::{
    Align, DisplayElement, DisplayElements, Font, ImageElement, Screen, TextElement,
};
use busylib::types::priority::Priority;
use busylib::{Client, ClientBuilder, HttpTransport};
use serde::Deserialize;

const APP: &str = "weather";
const DEFAULT_URL: &str = "http://10.0.4.20";
const FORECAST_URL: &str = "http://api.open-meteo.com/v1/forecast";
const AMSTERDAM: (f64, f64) = (52.3676, 4.9041);
const TIMEZONE: &str = "Europe/Amsterdam";
const REFRESH: Duration = Duration::from_secs(600);

const ICONS: [(&str, &[u8]); 7] = [
    (
        "cloud.png",
        include_bytes!("../../../assets/noto_emoji/cloud.png"),
    ),
    (
        "fog.png",
        include_bytes!("../../../assets/noto_emoji/fog.png"),
    ),
    (
        "partly.png",
        include_bytes!("../../../assets/noto_emoji/partly.png"),
    ),
    (
        "rain.png",
        include_bytes!("../../../assets/noto_emoji/rain.png"),
    ),
    (
        "snow.png",
        include_bytes!("../../../assets/noto_emoji/snow.png"),
    ),
    (
        "sun.png",
        include_bytes!("../../../assets/noto_emoji/sun.png"),
    ),
    (
        "thunder.png",
        include_bytes!("../../../assets/noto_emoji/thunder.png"),
    ),
];

#[derive(Debug, Deserialize)]
struct Forecast {
    current: CurrentWeather,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature_2m: f64,
    weather_code: u16,
}

#[derive(Debug, Clone, Copy)]
struct Condition {
    icon: &'static str,
    label: &'static str,
}

fn condition(weather_code: u16) -> Condition {
    let (icon, label) = match weather_code {
        0 => ("sun.png", "CLEAR"),
        1 | 2 => ("partly.png", "PARTLY CLOUDY"),
        3 => ("cloud.png", "OVERCAST"),
        45 | 48 => ("fog.png", "FOG"),
        51..=57 => ("rain.png", "DRIZZLE"),
        61..=67 | 80..=82 => ("rain.png", "RAIN"),
        71..=77 | 85 | 86 => ("snow.png", "SNOW"),
        95..=99 => ("thunder.png", "THUNDERSTORM"),
        _ => ("cloud.png", "UNKNOWN"),
    };

    Condition { icon, label }
}

async fn upload_icons<T: HttpTransport>(device: &Client<T>) -> Result<(), Box<dyn Error>> {
    for (name, bytes) in ICONS {
        device.assets().upload(APP, name, bytes).await?;
    }

    Ok(())
}

async fn fetch_current(http: &reqwest::Client) -> Result<CurrentWeather, Box<dyn Error>> {
    let (latitude, longitude) = AMSTERDAM;
    let url = format!(
        "{FORECAST_URL}?latitude={latitude}&longitude={longitude}\
         &current=temperature_2m,weather_code&timezone={TIMEZONE}"
    );

    let response = http.get(url).send().await?.error_for_status()?;
    let forecast: Forecast = serde_json::from_slice(&response.bytes().await?)?;

    Ok(forecast.current)
}

async fn draw<T: HttpTransport>(
    device: &Client<T>,
    current: &CurrentWeather,
) -> Result<(), Box<dyn Error>> {
    let condition = condition(current.weather_code);
    let reading = format!("{:.0}C {}", current.temperature_2m, condition.label);
    let timeout = REFRESH.as_secs() as u32 * 2;

    let frame = DisplayElements::new(APP)?
        .priority(Priority::new(10)?)
        .element(
            DisplayElement::builder("icon")?
                .screen(Screen::Front)
                .align(Align::TopLeft)
                .at(0, 0)
                .timeout_secs(timeout)
                .image(ImageElement::asset(condition.icon)?),
        )
        .element(
            DisplayElement::builder("reading")?
                .screen(Screen::Front)
                .align(Align::MidLeft)
                .at(18, 8)
                .timeout_secs(timeout)
                .text(
                    TextElement::new(reading.as_str(), Font::Normal)?
                        .width(52)
                        .scroll_rate(360)
                        .scroll_start_delay_ms(1500)
                        .scroll_repeat_delay_ms(3000),
                ),
        );

    device.assets().draw(&frame).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = env::var("BUSYBAR_URL").unwrap_or_else(|_| DEFAULT_URL.to_owned());

    // todo: now we have two http clients
    let device = ClientBuilder::new(&url)?
        .timeout(Duration::from_secs(5))
        .build_reqwest();
    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    upload_icons(&device).await?;

    loop {
        match fetch_current(&http).await {
            Ok(current) => draw(&device, &current).await?,
            Err(error) => eprintln!("failed to fetch the weather: {error}"),
        }

        tokio::time::sleep(REFRESH).await;
    }
}
