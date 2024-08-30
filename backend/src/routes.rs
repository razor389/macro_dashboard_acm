use warp::Filter;
use crate::handlers::{inflation::get_inflation, tbill::get_tbill, real_yield::get_real_yield};
use log::info;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    info!("Configuring routes...");

    // Define each route separately
    let inflation_route = warp::path!("api" / "v1" / "inflation")
        .and(warp::get())
        .and_then(get_inflation)
        .map(|reply| {
            info!("Inflation route was hit.");
            reply
        });

    let tbill_route = warp::path!("api" / "v1" / "tbill")
        .and(warp::get())
        .and_then(get_tbill)
        .map(|reply| {
            info!("T-bill route was hit.");
            reply
        });

    let real_yield_route = warp::path!("api" / "v1" / "real_yield")
        .and(warp::get())
        .and_then(get_real_yield)
        .map(|reply| {
            info!("Real yield route was hit.");
            reply
        });

    // Combine all routes
    info!("All routes configured successfully.");
    inflation_route.or(tbill_route).or(real_yield_route)
}
