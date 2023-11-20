use std::{path::Path, str::FromStr};

use actix_web::{web, App, HttpServer};
use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use csv_parse::CSVParser;
use dotenv::dotenv;
use log::{error, info, warn};
use log4rs;

mod constants;
mod csv_parse;
mod db_interactions;
mod event_routes;
mod types;
mod user_routes;

use event_routes::*;
use user_routes::*;

async fn update_db() {
    let mut csvparser = CSVParser::new(
        "latin1",
        Path::new("./input_files/Infopanel_new.csv").to_path_buf(),
    );

    match csvparser.read_file() {
        Ok(_) => info!("Successfully read csv file"),
        Err(e) => {
            error!("Could not read csv file: {}", e);
            return;
        }
    }

    match csvparser.parse_contents() {
        Ok(_) => info!("Successfully parsed csv file"),
        Err(e) => {
            error!("Could not parse csv file: {}", e);
            return;
        }
    }

    if let Some(events) = csvparser.get_events() {
        if let Ok(purge) = db_interactions::purge_events().await {
            warn!("Purged {} entries from event db.", purge.len());
            match db_interactions::create_many_events(events).await {
                Ok(_) => info!("Successfully created new events"),
                Err(e) => error!("Could not create new events: {}", e),
            }
        }
    }
}

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
                update_db().await;
                let tomorrow = Local::now() + Duration::hours(24);
                update_time = tomorrow.date_naive().and_time(specified_time);
            }
        }
    });

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("api/events")
                    .service(show_events)
                    .service(filter)
                    .service(filter_by_time)
                    .service(event_detail)
                    .service(delete_event),
            )
            .service(
                web::scope("api/users")
                    .service(show_users)
                    .service(user_detail),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
