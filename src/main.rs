use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
}

impl Forecast {
    async fn get(city: &str, country_code: &str) -> Result<Self, ExitFailure> {
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}",
            city,
            country_code,
            env::var("API_TOKEN").unwrap(),
        );
        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url).await?.json::<Forecast>().await?;
        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let response = Forecast::get(&args.city, &args.country_code).await?;

    print!(
        "\nCity: {}, Country code: {}\nTemperature: {}°C (feels like: {}°C)\nHumidity: {}%\nWind speed: {} km/h\n",
        args.city,
        args.country_code,
        (response.main.temp - 273.15).round(),
        (response.main.feels_like - 273.15).round(),
        response.main.humidity,
        response.wind.speed.round(),
    );
    Ok(())
}
