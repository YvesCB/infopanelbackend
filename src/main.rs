use axum::{routing::get, Router};

mod controller;
mod error;

use controller::{events, rooms, users};
use log::{info, warn};

#[tokio::main]
async fn main() {
    log4rs::init_file("log_config.yml", Default::default()).unwrap();

    let room_routes = Router::new()
        .route("/", get(rooms::get).post(rooms::post))
        .route("/{id}", get(rooms::get_by_id).delete(rooms::delete));

    let event_routes = Router::new()
        .route("/", get(events::get).post(events::post))
        .route("/{id}", get(events::get_by_id).delete(events::delete));

    let user_routes = Router::new()
        .route("/", get(users::get).post(users::post))
        .route("/{id}", get(users::get_by_id).delete(users::delete));

    let api_routes = Router::new()
        .nest("/rooms", room_routes)
        .nest("/events", event_routes)
        .nest("/users", user_routes);

    let app = Router::new().nest("/api", api_routes);

    let address = "127.0.0.1:8088";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    info!("Starting server on {address}");
    axum::serve(listener, app).await.unwrap();
}
