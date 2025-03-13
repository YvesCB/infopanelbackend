use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use super::room::Room;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")] // Don't include `id` when creating
    pub id: Option<RecordId>,
    pub pxid: u32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub department: String,
    pub classname: String,
    pub subject: String,
    pub teacher: String,
    pub room: RecordId, // foreign key
    pub modifiedat: Option<NaiveDateTime>,
    pub modifiedby: Option<String>,
    pub visible: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EventWithRoom {
    pub id: Option<RecordId>,
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
}
