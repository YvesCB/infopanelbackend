use std::sync::Arc;

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, RecordId, Surreal};

use crate::{Error, Result};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Room {
    pub id: RecordId,
    pub location: String,
    pub roomname: String,
    pub capacity: u32,
    pub createdby: Option<String>,
    pub createdat: Option<NaiveDateTime>,
    pub editedby: Option<String>,
    pub editedat: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoomForCreate {
    pub location: String,
    pub roomname: String,
    pub capacity: u32,
    pub createdby: Option<String>,
    pub createdat: Option<NaiveDateTime>,
    pub editedby: Option<String>,
    pub editedat: Option<NaiveDateTime>,
}

#[derive(Clone)]
pub struct RoomModelController {
    event_store: Arc<Surreal<Client>>,
}

impl RoomModelController {
    pub async fn new() -> Result<Self> {
        let db = super::init_surreal().await?;

        Ok(Self {
            event_store: Arc::new(db.into()),
        })
    }
}

impl RoomModelController {
    pub async fn create(&self, mut room_fc: RoomForCreate, user: String) -> Result<Room> {
        room_fc.createdby = Some(user.clone());
        room_fc.createdat = Some(Utc::now().naive_local());
        let cr_room: Option<Room> = self
            .event_store
            .create(super::ROOMS)
            .content(room_fc)
            .await?;

        match cr_room {
            Some(e) => Ok(e),
            None => Err(Error::DataBaseCouldNotInsert),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Room>> {
        let events: Vec<Room> = self.event_store.select(super::ROOMS).await?;

        Ok(events)
    }
}
