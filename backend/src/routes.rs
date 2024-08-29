use warp::Filter;
use crate::handlers::{inflation::get_inflation, tbill::get_tbill, real_yield::get_real_yield};

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("api").and(
        warp::path("v1")
            .and(
                warp::path("inflation").and(warp::get()).and_then(get_inflation)
                .or(warp::path("tbill").and(warp::get()).and_then(get_tbill))
                .or(warp::path("real_yield").and(warp::get()).and_then(get_real_yield))
            )
    )
}
