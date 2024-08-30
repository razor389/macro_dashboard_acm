use warp::reply::Json;
use warp::Rejection;
use crate::services::bls::fetch_inflation_data;
use log::{info, error};

pub async fn get_inflation() -> Result<Json, Rejection> {
    info!("Handling request to get inflation data.");

    match fetch_inflation_data().await {
        Ok(inflation) => {
            info!("Successfully fetched inflation data: {}", inflation);
            Ok(warp::reply::json(&inflation))
        }
        Err(e) => {
            error!("Failed to fetch inflation data: {}", e);
            Err(warp::reject::not_found())
        }
    }
}
