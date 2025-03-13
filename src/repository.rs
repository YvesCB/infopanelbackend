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

pub async fn select_events() -> Result<Vec<model::event::Event>, Error> {
    let events: Vec<model::event::Event> = DB.select(EVENT).await?;

    Ok(events)
}

pub async fn insert_event(event: model::event::Event) -> Result<(), Error> {
    let created_event: Option<model::event::Event> = DB.create(EVENT).content(event).await?;

    dbg!(created_event);

    Ok(())
}
