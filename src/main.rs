use axum::Router;
use dotenv::dotenv;
use std::net::SocketAddr;

mod serp_controller;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/autocomplete", axum::routing::post(serp_controller::get_auto_complete))
        .route("/hotels", axum::routing::post(serp_controller::get_serp_hotels))
        .route("/region", axum::routing::post(serp_controller::get_serp_region));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
