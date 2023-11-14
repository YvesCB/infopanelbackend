use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse};

use crate::db_interactions;

#[get("")]
pub async fn show_events() -> HttpResponse {
    if let Ok(all_events) = db_interactions::get_all_events().await {
        HttpResponse::Ok()
            .status(StatusCode::OK)
            .json(web::Json(all_events))
    } else {
        HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Error with db.")
    }
}

#[get("/{id}")]
pub async fn event_detail(path: web::Path<(u64,)>) -> HttpResponse {
    if let Ok(event) = db_interactions::get_event(path.into_inner().0).await {
        match event {
            Some(e) => HttpResponse::Ok().status(StatusCode::OK).json(web::Json(e)),
            None => HttpResponse::Ok()
                .status(StatusCode::NOT_FOUND)
                .body("Event not found."),
        }
    } else {
        HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Error with db.")
    }
}
