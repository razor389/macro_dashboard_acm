mod handlers;
mod services;
mod routes;
use env_logger;
use log::{info, error};

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init(); 
    info!("Logger initialized. Starting the application...");

    // Set up the routes
    let api = routes::routes();
    info!("Routes configured successfully.");

    // Start the Warp server
    info!("Starting the Warp server on http://127.0.0.1:3030");
    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;

    info!("Server shut down gracefully.");
}

