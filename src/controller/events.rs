use axum::{extract::Path, response::IntoResponse};
use http::status::StatusCode;

pub async fn get() -> impl IntoResponse {
    (StatusCode::OK, "get events")
}

pub async fn post() -> impl IntoResponse {
    (StatusCode::CREATED, "posted event")
}

pub async fn get_by_id(Path(event_id): Path<u32>) -> impl IntoResponse {
    (StatusCode::OK, format!("get event with id: {}", event_id))
}

pub async fn delete(Path(event_id): Path<u32>) -> impl IntoResponse {
    (
        StatusCode::ACCEPTED,
        format!("delete event with id: {}", event_id),
    )
}
