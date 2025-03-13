use log::{error, info};
use std::sync::LazyLock;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::error::Error;
use crate::model;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

const EVENT: &str = "event";
const ROOM: &str = "room";

pub async fn setup_db() -> Result<(), Error> {
    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("juventus").use_db("infopanel").await?;

    Ok(())
}

pub async fn select_events() -> Result<Vec<model::event::EventWithRoom>, Error> {
    let sql = "SELECT *, room.* FROM event FETCH room;";
    let events: Vec<model::event::EventWithRoom> = DB.query(sql).await?.take(0)?;

    Ok(events)
}

pub async fn insert_event(event: model::event::Event) -> Result<(), Error> {
    let foundrooms: Option<model::room::Room> = DB.select(&event.room).await?;

    match foundrooms {
        Some(_) => {
            let created_event: Option<model::event::Event> =
                DB.create(EVENT).content(event).await?;
            match created_event {
                Some(event) => info!("Created event:\n{:?}", event),
                None => error!("Could not create event"),
            }
        }
        None => {
            return Err(Error::InvalidData);
        }
    }

    Ok(())
}

pub async fn select_rooms() -> Result<Vec<model::room::Room>, Error> {
    let rooms: Vec<model::room::Room> = DB.select(ROOM).await?;

    Ok(rooms)
}

pub async fn insert_room(room: model::room::Room) -> Result<(), Error> {
    // Check if a room with the same location and roomname already exists
    let existing: Vec<model::room::Room> = DB
        .query("SELECT * FROM room WHERE location = $location AND roomname = $roomname")
        .bind(("location", room.location.clone()))
        .bind(("roomname", room.roomname.clone()))
        .await?
        .take(0)?;

    Ok(())
}
