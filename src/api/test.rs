use actix_web::{get, HttpRequest, HttpResponse};

#[get("")]
pub async fn test_header(req: HttpRequest) -> HttpResponse {
    dbg!(req);
    HttpResponse::Ok().body("Good job")
}
