use actix_web::{get, web, HttpResponse};

#[get("")]
pub async fn show_events() -> HttpResponse {
    HttpResponse::Ok().body("Show events")
}

#[get("/{id}")]
pub async fn event_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Event detail: {}", path.into_inner().0))
}
