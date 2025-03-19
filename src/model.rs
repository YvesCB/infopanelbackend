use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::Result;

pub mod event;
pub mod room;
pub mod user;

const DBNS: &str = "juventus";
const DBNAME: &str = "infopanel";

// -- Tables
const EVENTS: &str = "events";
const ROOMS: &str = "rooms";

pub async fn init_surreal() -> Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns(DBNS).use_db(DBNAME).await?;

    Ok(db)
}
