use log::{info, error};
// backend/src/handlers/long_term.rs
use warp::reply::Json;
use warp::Rejection;
use serde::Serialize;
use crate::services::{
    treasury_long::{fetch_20y_bond_yield, fetch_20y_tips_yield},
    treasury::fetch_tbill_data,
};

#[derive(Serialize)]
pub struct LongTermRates {
    bond_yield: f64,
    tips_yield: f64,
    market_inflation: f64,
    estimated_inflation: f64,
    real_tbill: f64,
    horizon_premium: f64,
    estimated_horizon_premium: f64,
    delta_inflation: f64,
    delta_horizon: f64,
    estimated_returns: f64,
}

pub async fn get_long_term_rates() -> Result<Json, Rejection> {
    // Fetch all required rates
    let bond_yield = match fetch_20y_bond_yield().await {
        Ok(y) => {
            info!("Nominal 20y yield is: {}", y);
            y
        },
        Err(e) => {
            error!("Nominal 20y  yield error: {}", e);
            return Err(warp::reject::not_found());
        }
    };
    
    let tips_yield = match fetch_20y_tips_yield().await {
        Ok(y) => {
            info!("20y TIPS yield is: {}", y);
            y
        },
        Err(e) => {
            error!("20y TIPS yield error: {}", e);
            // If you want the route to fail entirely, keep returning the error:
            //return Err(warp::reject::not_found());
            
            // Or if you want to skip TIPS:
            0.0
        }
    };
    
    let real_tbill = fetch_tbill_data().await.map_err(|_| warp::reject::not_found())?;
    
    // Calculate derived values
    let market_inflation = bond_yield - tips_yield;
    let estimated_inflation = 2.5; // Free parameter
    let horizon_premium = tips_yield - real_tbill;
    let estimated_horizon_premium = 1.0; // Free parameter
    
    let delta_inflation = market_inflation - estimated_inflation;
    let delta_horizon = horizon_premium - estimated_horizon_premium;
    let estimated_returns = bond_yield + delta_inflation + delta_horizon;
    
    let rates = LongTermRates {
        bond_yield,
        tips_yield,
        market_inflation,
        estimated_inflation,
        real_tbill,
        horizon_premium,
        estimated_horizon_premium,
        delta_inflation,
        delta_horizon,
        estimated_returns,
    };
    
    Ok(warp::reply::json(&rates))
}