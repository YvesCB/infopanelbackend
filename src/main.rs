mod api;
mod constants;
mod model;
mod util;

use actix_web::{web, App, HttpServer};
use chrono::{Duration, Local, NaiveDateTime, NaiveTime};
use dotenv::dotenv;
use log4rs;

use api::event::{
    create_new_event, delete_event, event_detail, filter, filter_by_time, show_events, update_db,
};
use api::test::test_header;
use api::user::{show_users, user_detail};
use model::auth::Auth;
use util::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log_config.yml", Default::default()).unwrap();
    dotenv().ok();

    let tomorrow = Local::now() + Duration::hours(24);
    let mut specified_time =
        NaiveTime::parse_from_str(&std::env::var("REFRESH_TIME").unwrap(), "%H:%M").unwrap();

    let mut update_time: NaiveDateTime = tomorrow.date_naive().and_time(specified_time);

    db_interactions::initiate_db()
        .await
        .expect("Could not connect to db, aborting.");

    let interval_duration = tokio::time::Duration::from_secs(60);
    let mut interval_stream = tokio::time::interval(interval_duration);

    tokio::spawn(async move {
        loop {
            interval_stream.tick().await;
            let now = Local::now();
            let new_specified_time =
                NaiveTime::parse_from_str(&std::env::var("REFRESH_TIME").unwrap(), "%H:%M")
                    .unwrap();
            if new_specified_time != specified_time {
                update_time = update_time.date().and_time(specified_time);
                specified_time = new_specified_time;
            }

            dbg!(&update_time);

            if now.naive_local() > update_time {
                db_interactions::update_db().await;
                let tomorrow = Local::now() + Duration::hours(24);
                update_time = tomorrow.date_naive().and_time(specified_time);
            }
        }
    });

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("api/events")
                    .wrap(Auth)
                    .service(show_events)
                    .service(filter)
                    .service(filter_by_time)
                    .service(create_new_event)
                    .service(update_db)
                    .service(event_detail)
                    .service(delete_event),
            )
            .service(
                web::scope("api/users")
                    .service(show_users)
                    .service(user_detail),
            )
            .service(web::scope("api/test").service(test_header))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
