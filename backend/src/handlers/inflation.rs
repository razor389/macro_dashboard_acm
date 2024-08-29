use warp::reply::Json;
use warp::Rejection;
use crate::services::bls::fetch_inflation_data;

pub async fn get_inflation() -> Result<Json, Rejection> {
    match fetch_inflation_data().await {
        Ok(inflation) => Ok(warp::reply::json(&inflation)),
        Err(_) => Err(warp::reject::not_found()),
    }
}
