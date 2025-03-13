use axum::{extract::Path, response::IntoResponse};
use http::status::StatusCode;

pub async fn get() -> impl IntoResponse {
    (StatusCode::OK, "get rooms")
}

pub async fn post() -> impl IntoResponse {
    (StatusCode::CREATED, "posted room")
}

pub async fn get_by_id(Path(event_id): Path<u32>) -> impl IntoResponse {
    (StatusCode::OK, format!("get room with id: {}", event_id))
}

pub async fn delete(Path(event_id): Path<u32>) -> impl IntoResponse {
    (
        StatusCode::ACCEPTED,
        format!("delete room with id: {}", event_id),
    )
}
