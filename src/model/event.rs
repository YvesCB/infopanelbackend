use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, RecordId, Surreal};

use crate::{Error, Result};

use super::room::Room;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: RecordId,
    pub pxid: u32,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub department: String,
    pub classname: String,
    pub subject: String,
    pub teacher: String,
    pub room: Room,
    pub modifiedat: Option<DateTime<Utc>>,
    pub modifiedby: Option<String>,
    pub visible: bool,
    pub createdby: Option<String>,
    pub createdat: Option<DateTime<Utc>>,
    pub editedby: Option<String>,
    pub editedat: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventForCreate {
    pub pxid: u32,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub department: String,
    pub classname: String,
    pub subject: String,
    pub teacher: String,
    pub room: String, // foreign key
    pub visible: bool,
    pub createdby: Option<String>,
    pub createdat: Option<DateTime<Utc>>,
    pub editedby: Option<String>,
    pub editedat: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub struct EventModelController {
    event_store: Arc<Surreal<Client>>,
}

impl EventModelController {
    pub async fn new() -> Result<Self> {
        let db = super::init_surreal().await?;

        Ok(Self {
            event_store: Arc::new(db.into()),
        })
    }
}

impl EventModelController {
    pub async fn create(&self, mut event_fc: EventForCreate, user: String) -> Result<Event> {
        event_fc.createdby = Some(user.clone());
        event_fc.createdat = Some(Utc::now());

        let roomid = event_fc
            .room
            .split(':')
            .nth(1)
            .map(|s| s.to_string())
            .unwrap_or_default();

        let sql = "
            CREATE event SET 
            pxid = $pxid, 
            start = <datetime> $start, 
            end = <datetime> $end, 
            department = $dep, 
            classname = $classname, 
            subject = $subject, 
            teacher = $teacher, 
            room = type::thing('room', $roomid),
            visible = $visible;
        ";

        let mut result = self
            .event_store
            .query(sql)
            .bind(("pxid", event_fc.pxid))
            .bind(("start", event_fc.start))
            .bind(("end", event_fc.end))
            .bind(("dep", event_fc.department))
            .bind(("classname", event_fc.classname))
            .bind(("subject", event_fc.subject))
            .bind(("teacher", event_fc.teacher))
            .bind(("roomid", roomid))
            .bind(("visible", event_fc.visible))
            .await?;

        let created_event: Option<EventForCreate> = result.take(0)?;
        dbg!(&created_event);

        let created_id = match created_event {
            Some(thing) => thing,
            None => return Err(Error::DataBaseCouldNotInsert),
        };

        let mut fetch_result = self
            .event_store
            .query("SELECT *, room.* FROM type::thing('event', $id);")
            .bind(("id", created_id))
            .await?;

        let cr_event: Option<Event> = fetch_result.take(0)?;

        match cr_event {
            Some(e) => Ok(e),
            None => Err(Error::DataBaseCouldNotInsert),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Event>> {
        let events: Vec<Event> = self.event_store.select(super::ROOMS).await?;

        Ok(events)
    }
}
