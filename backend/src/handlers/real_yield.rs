use warp::reply::Json;
use warp::Rejection;
use crate::services::bls::fetch_inflation_data;
use crate::services::treasury::fetch_tbill_data;

pub async fn get_real_yield() -> Result<Json, Rejection> {
    let inflation = fetch_inflation_data().await.map_err(|_| warp::reject::not_found())?;
    let tbill_rate = fetch_tbill_data().await.map_err(|_| warp::reject::not_found())?;

    let real_yield = tbill_rate - inflation;

    Ok(warp::reply::json(&real_yield))
}
