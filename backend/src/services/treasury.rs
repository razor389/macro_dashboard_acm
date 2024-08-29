use reqwest;
use scraper::{Html, Selector};
use std::num::ParseFloatError;

pub async fn fetch_tbill_data() -> Result<f64, Box<dyn std::error::Error>> {
    let url = "https://home.treasury.gov/resource-center/data-chart-center/interest-rates/TextView?type=daily_treasury_bill_rates&field_tdr_date_value=2024";
    let body = reqwest::get(url).await?.text().await?;
    
    let document = Html::parse_document(&body);
    let selector = Selector::parse("table tbody tr:last-child td:last-child").unwrap();

    if let Some(element) = document.select(&selector).next() {
        let tbill_rate = element.text().collect::<Vec<_>>().concat();
        return tbill_rate.parse::<f64>()
            .map_err(|e: ParseFloatError| {
                format!("Failed to parse T-bill rate: {}", e).into()
            });
    }

    Err("No T-bill data found".into())
}
