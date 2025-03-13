use axum::{extract::Path, response::IntoResponse};
use http::status::StatusCode;

pub async fn get() -> impl IntoResponse {
    (StatusCode::OK, "get users")
}

pub async fn post() -> impl IntoResponse {
    (StatusCode::CREATED, "posted user")
}

pub async fn get_by_id(Path(event_id): Path<u32>) -> impl IntoResponse {
    (StatusCode::OK, format!("get user with id: {}", event_id))
}

pub async fn delete(Path(event_id): Path<u32>) -> impl IntoResponse {
    (
        StatusCode::ACCEPTED,
        format!("delete user with id: {}", event_id),
    )
}
