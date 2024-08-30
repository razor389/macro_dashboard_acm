mod handlers;
mod services;
mod routes;
use env_logger;
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init(); 
    let api = routes::routes();
    info!("Starting the application");
    
    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
