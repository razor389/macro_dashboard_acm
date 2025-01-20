// backend/src/services/treasury_long.rs
use reqwest;
use scraper::{Html, Selector};
use log::{info, error};
use std::error::Error;

pub async fn fetch_20y_bond_yield() -> Result<f64, Box<dyn Error>> {
    let url = "https://home.treasury.gov/resource-center/data-chart-center/interest-rates/daily-treasury-rates.csv/2024?type=daily_treasury_yield_curve&field_tdr_date_value=2024&page&_format=csv";
    info!("Fetching 20-year bond yield from URL: {}", url);
    
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    
    // Parse CSV content
    let mut reader = csv::Reader::from_reader(body.as_bytes());
    
    // Get the most recent row (last non-empty row)
    let mut latest_yield: Option<f64> = None;
    
    for result in reader.records() {
        if let Ok(record) = result {
            // The 20-year yield should be in a specific column (adjust index based on CSV structure)
            if let Some(yield_str) = record.get(13) { // Adjust index as needed
                if !yield_str.trim().is_empty() {
                    if let Ok(yield_value) = yield_str.trim().parse::<f64>() {
                        latest_yield = Some(yield_value);
                    }
                }
            }
        }
    }
    
    latest_yield.ok_or_else(|| "No 20-year bond yield data found.".into())
}

pub async fn fetch_20y_tips_yield() -> Result<f64, Box<dyn Error>> {
    let url = "https://home.treasury.gov/resource-center/data-chart-center/interest-rates/daily-treasury-rates.csv/2024?type=daily_treasury_real_yield_curve&field_tdr_date_value=2024&page&_format=csv";
    info!("Fetching 20-year TIPS yield from URL: {}", url);
    
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    
    // Parse CSV content
    let mut reader = csv::Reader::from_reader(body.as_bytes());
    
    // Get the most recent row (last non-empty row)
    let mut latest_yield: Option<f64> = None;
    
    for result in reader.records() {
        if let Ok(record) = result {
            // The 20-year TIPS yield should be in a specific column (adjust index based on CSV structure)
            if let Some(yield_str) = record.get(4) { // Adjust index as needed
                if !yield_str.trim().is_empty() {
                    if let Ok(yield_value) = yield_str.trim().parse::<f64>() {
                        latest_yield = Some(yield_value);
                    }
                }
            }
        }
    }
    
    latest_yield.ok_or_else(|| "No 20-year TIPS yield data found.".into())
}
