use axum::{routing::get, Router};

mod controller;
use controller::events;

#[tokio::main]
async fn main() {
    let event_routes = Router::new()
        .route("/", get(events::get).post(events::post))
        .route("/{id}", get(events::get_by_id).delete(events::delete));

    let user_routes = Router::new()
        .route(
            "/",
            get(|| async { "get users" }).post(|| async { "post user" }),
        )
        .route(
            "/{id}",
            get(|| async { "get user by id" }).delete(|| async { "delete user by id" }),
        );

    let api_routes = Router::new()
        .nest("/events", event_routes)
        .nest("/users", user_routes);

    let app = Router::new().nest("/api", api_routes);

    let address = "127.0.0.1:8088";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
