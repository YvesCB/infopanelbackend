use log::warn;
use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::constants::*;
use crate::types::*;

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

/// Initiate the connection to the surrealdb server
///
/// Uses the namespace `juventus` and the database name `infopanel`.
pub async fn initiate_db() -> surrealdb::Result<()> {
    DB.connect::<Ws>("localhost:8000").await?;
    warn!("Connected to DB at localhost:8000");

    let dbuser = std::env::var("SURREAL_INFO_USER").expect("missing SURREAL_INFO_USER");
    let dbpass = std::env::var("SURREAL_INFO_PASS").expect("missing SURREAL_INFO_PASS");

    DB.signin(Root {
        username: &dbuser,
        password: &dbpass,
    })
    .await?;
    warn!("Signed into DB");

    DB.use_ns("juventus").use_db("infopanel").await?;
    warn!("Using ns {} and db {}", "juventus", "infopanel");

    Ok(())
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
    match existing_event {
        Some(_) => Ok(None),
        None => {
            let created_event: Option<Event> = DB
                .create((EVENT_TABLE, event.event_id))
                .content(event)
                .await?;

            Ok(created_event)
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

/// Remove an event from the database by id
///
/// Returns `Some(Event)` if the event with the `id` provided was found and deleted.
/// If the event with the given `id` isn't found, `None` is returned.
/// Can return a `surrealdb::Error`.
pub async fn delete_event(id: u64) -> Result<Option<Event>, surrealdb::Error> {
    let event: Option<Event> = DB.delete((EVENT_TABLE, id)).await?;

    Ok(event)
}
