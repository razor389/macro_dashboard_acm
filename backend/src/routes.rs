use warp::Filter;
use crate::handlers::{inflation::get_inflation, tbill::get_tbill, real_yield::get_real_yield};

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Define each route separately
    let inflation_route = warp::path!("api" / "v1" / "inflation")
        .and(warp::get())
        .and_then(get_inflation);

    let tbill_route = warp::path!("api" / "v1" / "tbill")
        .and(warp::get())
        .and_then(get_tbill);

    let real_yield_route = warp::path!("api" / "v1" / "real_yield")
        .and(warp::get())
        .and_then(get_real_yield);

    // Combine all routes
    inflation_route.or(tbill_route).or(real_yield_route)
}
