use reqwest;
use serde::Deserialize;
use std::error::Error as StdError;
use std::env;
use dotenv::dotenv;
use std::fmt;

#[derive(Deserialize)]
struct BlsResponse {
    results: Results,
}

#[derive(Deserialize)]
struct Results {
    series: Vec<Series>,
}

#[derive(Deserialize)]
struct Series {
    data: Vec<DataPoint>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct DataPoint {
    year: String,
    period: String,
    value: String,
}

// Define a custom error type
#[derive(Debug)]
struct DataFetchError {
    details: String,
}

impl DataFetchError {
    fn new(msg: &str) -> DataFetchError {
        DataFetchError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for DataFetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl StdError for DataFetchError {}

pub async fn fetch_inflation_data() -> Result<f64, Box<dyn StdError>> {
    dotenv().ok();  // Load environment variables from .env file
    
    let api_key = env::var("BLS_API_KEY").expect("BLS_API_KEY must be set");
    let url = format!(
        "https://api.bls.gov/publicAPI/v2/timeseries/data/CUUR0000SA0?registrationkey={}",
        api_key
    );
    let resp = reqwest::get(&url).await?.json::<BlsResponse>().await?;

    if let Some(latest_data) = resp.results.series.first().and_then(|series| series.data.first()) {
        let inflation_rate: f64 = latest_data.value.parse().unwrap_or(0.0);
        return Ok(inflation_rate);
    }

    Err(Box::new(DataFetchError::new("No data found")))
}
