use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log4rs;

mod constants;
mod db_interactions;
mod event_routes;
mod types;
mod user_routes;

use event_routes::*;
use user_routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log_config.yml", Default::default()).unwrap();
    dotenv().ok();

    db_interactions::initiate_db()
        .await
        .expect("Could not connect to db, aborting.");

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/events")
                    .service(show_events)
                    .service(event_detail),
            )
            .service(
                web::scope("/users")
                    .service(show_users)
                    .service(user_detail),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
