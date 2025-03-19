use std::sync::Arc;

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use surrealdb::{engine::remote::ws::Client, RecordId, Surreal};

use crate::{Error, Result};

use super::room::Room;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: RecordId,
    pub pxid: u32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub department: String,
    pub classname: String,
    pub subject: String,
    pub teacher: String,
    pub room: Room,
    pub modifiedat: Option<NaiveDateTime>,
    pub modifiedby: Option<String>,
    pub visible: bool,
    pub createdby: Option<String>,
    pub createdat: Option<NaiveDateTime>,
    pub editedby: Option<String>,
    pub editedat: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventForCreate {
    pub pxid: u32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub department: String,
    pub classname: String,
    pub subject: String,
    pub teacher: String,
    pub room: String, // foreign key
    pub visible: bool,
    pub createdby: Option<String>,
    pub createdat: Option<NaiveDateTime>,
    pub editedby: Option<String>,
    pub editedat: Option<NaiveDateTime>,
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
        event_fc.createdat = Some(Utc::now().naive_local());
        let sql = "
            CREATE $table SET 
            pxid = $pxid, 
            start = $start, 
            end = $end, 
            department = $dep, 
            classname = $classname, 
            subject = $subject, 
            teacher = $teacher, 
            room = SELECT * FROM $roomtable WHERE id = $roomid,
            visible = $visible;
        ";

        let mut result = self
            .event_store
            .query(sql)
            .bind(("table", super::EVENTS))
            .bind(("pxid", event_fc.pxid))
            .bind(("start", event_fc.start))
            .bind(("end", event_fc.end))
            .bind(("dep", event_fc.department))
            .bind(("classname", event_fc.classname))
            .bind(("subject", event_fc.subject))
            .bind(("teacher", event_fc.teacher))
            .bind(("roomtable", super::ROOMS))
            .bind(("roomid", event_fc.room))
            .bind(("visible", event_fc.visible))
            .await?;
        let cr_event: Option<Event> = result.take(0)?;
        //let cr_event: Option<Event> = self
        //    .event_store
        //    .create(super::EVENTS)
        //    .content(event_fc)
        //    .await?;

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
