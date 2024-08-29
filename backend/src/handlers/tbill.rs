use warp::reply::Json;
use warp::Rejection;
use crate::services::treasury::fetch_tbill_data;

pub async fn get_tbill() -> Result<Json, Rejection> {
    match fetch_tbill_data().await {
        Ok(tbill_rate) => Ok(warp::reply::json(&tbill_rate)),
        Err(_) => Err(warp::reject::not_found()),
    }
}
