use reqwest;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;
use std::error::Error as StdError;
use std::fmt;

#[derive(Deserialize, Debug)]
struct BlsResponse {
    status: String,
    Results: Results,  // Update here to match the JSON structure
}

#[derive(Deserialize, Debug)]
struct Results {  // New struct to capture the "Results" object
    series: Vec<Series>,
}

#[derive(Deserialize, Debug)]
struct Series {
    seriesID: String,
    data: Vec<DataPoint>,
}

#[derive(Deserialize, Debug)]
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
    println!("API Key: {}", api_key);  // Debugging: Check if API key is loaded
    
    let url = "https://api.bls.gov/publicAPI/v1/timeseries/data/";
    println!("Request URL: {}", url);  // Debugging: Check the request URL

    let request_body = serde_json::json!({
        "seriesid": ["CUUR0000SA0"],
        "registrationkey": api_key
    });
    println!("Request Body: {:?}", request_body);  // Debugging: Check the request body

    let client = reqwest::Client::new();
    let response = client.post(url)
        .json(&request_body)
        .send()
        .await?;
        
    let response_text = response.text().await?;
    println!("Response Text: {}", response_text);  // Debugging: Print the full response body
    
    let resp: BlsResponse = serde_json::from_str(&response_text)?;
    println!("Parsed Response: {:?}", resp);  // Debugging: Check the parsed response

    if let Some(series) = resp.Results.series.first() {  // Update here to match the new struct
        println!("Series Data: {:?}", series.data);  // Debugging: Check the series data

        // Get the most recent data point
        if let Some(current_data) = series.data.first() {
            let current_year = &current_data.year;
            let current_period = &current_data.period;
            let current_value: f64 = current_data.value.parse().unwrap_or(0.0);

            println!("Current Year: {}, Current Period: {}, Current Value: {}", current_year, current_period, current_value);  // Debugging: Check the current data

            // Find the data point from the same month last year
            if let Some(last_year_data) = series.data.iter().find(|d| {
                &d.year == &(current_year.parse::<i32>().unwrap() - 1).to_string() && &d.period == current_period
            }) {
                let last_year_value: f64 = last_year_data.value.parse().unwrap_or(0.0);

                println!("Last Year Value: {}", last_year_value);  // Debugging: Check the last year's data

                // Calculate the yearly percentage change
                let percentage_change = ((current_value - last_year_value) / last_year_value) * 100.0;
                println!("Yearly Percentage Change: {}", percentage_change);  // Debugging: Check the calculated percentage change
                return Ok(percentage_change);
            } else {
                println!("No data found for the same month last year.");  // Debugging: If last year's data is missing
            }
        } else {
            println!("No current data found.");  // Debugging: If current data is missing
        }
    } else {
        println!("No series data found.");  // Debugging: If series data is missing
    }

    Err(Box::new(DataFetchError::new("No data found")))
}
