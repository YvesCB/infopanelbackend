use std::path::Path;

use log::{error, info, warn};
use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::constants::*;
use crate::model::event::*;
use crate::util::*;

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn create_new_event() {
        dotenv().ok();
        initiate_db().await.unwrap();

        let d = NaiveDate::from_ymd_opt(2023, 11, 14).unwrap();
        let t = NaiveTime::from_hms_opt(10, 15, 0).unwrap();
        let from_datetime = NaiveDateTime::new(d, t);
        let to_datetime = NaiveDateTime::new(d, t.with_hour(11).unwrap());
        let new_event = Event {
            event_id: 10239745,
            from_datetime,
            to_datetime,
            room: "201".to_string(),
            subject: "Test".to_string(),
            teacher: "Test".to_string(),
            visible: true,
            building: "Lagerstrasse 102".to_string(),
            department: "SJS".to_string(),
            class_name: "MP MPA 2208 B 02".to_string(),
            modified_at: None,
            modified_by: None,
        };

        let created_event = create_event(new_event.clone()).await.unwrap();
        match created_event {
            Some(e) => assert_eq!(e, new_event),
            None => {
                assert!(get_event(new_event.event_id).await.unwrap().is_some())
            }
        }
    }
}

/// Perform a sunc of the database
pub async fn update_db() -> DBUpdateData {
    let mut csvparser = csv_parse::CSVParser::new(
        "latin1",
        Path::new("./input_files/Infopanel_new.csv").to_path_buf(),
    );

    match csvparser.read_file() {
        Ok(_) => info!("Successfully read csv file"),
        Err(e) => {
            error!("Could not read csv file: {}", e);
            return DBUpdateData {
                deleted_events: 0,
                answer_string: String::from("Could not read csv file"),
                sucess: false,
            };
        }
    }

    match csvparser.parse_contents() {
        Ok(_) => info!("Successfully parsed csv file"),
        Err(e) => {
            error!("Could not parse csv file: {}", e);
            return DBUpdateData {
                deleted_events: 0,
                answer_string: String::from("Could not parse csv file"),
                sucess: false,
            };
        }
    }

    if let Some(events) = csvparser.get_events() {
        if let Ok(purge) = db_interactions::purge_events().await {
            warn!("Purged {} entries from event db.", purge.len());
            match db_interactions::create_many_events(events).await {
                Ok(_) => {
                    info!("Successfully created new events");
                    return DBUpdateData {
                        deleted_events: purge.len(),
                        answer_string: String::from("Successfully updated database with new data."),
                        sucess: true,
                    };
                }
                Err(e) => {
                    error!("Could not create new events: {}", e);
                    return DBUpdateData {
                        deleted_events: purge.len(),
                        answer_string: String::from("Purged db, but cannot read in new data."),
                        sucess: false,
                    };
                }
            }
        }
    }

    return DBUpdateData {
        deleted_events: 0,
        answer_string: String::from("Purged db, but cannot read in new data."),
        sucess: false,
    };
}

/// Initiate the connection to the surrealdb server
///
/// Uses the namespace `juventus` and the database name `infopanel`.
pub async fn initiate_db() -> surrealdb::Result<()> {
    DB.connect::<Ws>("localhost:8000").await?;
    info!("Connected to DB at localhost:8000");

    let dbuser = std::env::var("SURREAL_INFO_USER").expect("missing SURREAL_INFO_USER");
    let dbpass = std::env::var("SURREAL_INFO_PASS").expect("missing SURREAL_INFO_PASS");

    DB.signin(Root {
        username: &dbuser,
        password: &dbpass,
    })
    .await?;
    info!("Signed into DB");

    DB.use_ns("juventus").use_db("infopanel").await?;
    info!("Using ns {} and db {}", "juventus", "infopanel");

    Ok(())
}

/// Add a list of events to the database
///
/// Returns a simple result with no content. If the transaction fails `surrealdb::Error` will be
/// returned.
pub async fn create_many_events(events: &Vec<Event>) -> Result<(), surrealdb::Error> {
    for event in events {
        let existing_event: Option<Event> = DB.select((EVENT_TABLE, event.event_id)).await?;
        match existing_event {
            Some(_) => {}
            None => {
                let _created_event: Option<Event> = DB
                    .create((EVENT_TABLE, event.event_id))
                    .content(event)
                    .await?;
            }
        }
    }

    Ok(())
}

/// Delete every event in the database
///
/// Returns the deleted events or a `surrealdb::Error`.
pub async fn purge_events() -> Result<Vec<Event>, surrealdb::Error> {
    let deleted_events: Vec<Event> = DB.delete(EVENT_TABLE).await?;

    Ok(deleted_events)
}

/// Create a new event in the database
///
/// Returns `Some(Event)` if the event was successfully created in the database.
/// If the event with the id already exists in the db, it will return `None`
/// Can return a `surrealdb::Error`.
pub async fn create_event(event: Event) -> Result<Option<Event>, surrealdb::Error> {
    // we check for existing record like this because calling create on an existing id just throws
    // an error and it makes it harder to distinguish from another type of error.
    let existing_event: Option<Event> = DB.select((EVENT_TABLE, event.event_id)).await?;
    dbg!(event.event_id);
    match existing_event {
        Some(_) => Ok(None),
        None => {
            let created_event: Result<Option<Event>, surrealdb::Error> = DB
                .create((EVENT_TABLE, event.event_id))
                .content(event)
                .await;
            match created_event {
                Ok(ce) => Ok(ce),
                Err(e) => {
                    println!("{:?}", e);
                    return Err(e);
                }
            }

            // Ok(created_event)
        }
    }
}

/// Get a event from the database by id
///
/// Returns `Some(Event)` if the event with the `id` provided was found.
/// If the event with the given `id` isn't found, `None` is returned.
/// Can return a `surrealdb::Error`.
pub async fn get_event(id: u64) -> Result<Option<Event>, surrealdb::Error> {
    let event: Option<Event> = DB.select((EVENT_TABLE, id)).await?;

    Ok(event)
}

/// Get all the events in the table sorted by `from_datetime`
///
/// Returns a `Vec<Event>` which can be empty.
/// Can return a `surrealdb::Error`.
pub async fn get_all_events() -> Result<Vec<Event>, surrealdb::Error> {
    let mut events: Vec<Event> = DB.select(EVENT_TABLE).await?;
    events.sort_by_key(|a| a.from_datetime);

    Ok(events)
}

/// Remove an event from the database by id
///
/// Returns `Some(Event)` if the event with the `id` provided was found and deleted.
/// If the event with the given `id` isn't found, `None` is returned.
/// Can return a `surrealdb::Error`.
pub async fn delete_event(id: u64) -> Result<Option<Event>, surrealdb::Error> {
    let event: Option<Event> = DB.delete((EVENT_TABLE, id)).await?;

    Ok(event)
}
