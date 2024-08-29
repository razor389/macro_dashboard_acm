mod handlers;
mod services;
mod routes;

#[tokio::main]
async fn main() {
    let api = routes::routes();

    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
