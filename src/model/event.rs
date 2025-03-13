use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::room::Room;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Event {
    pub pxid: u32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub department: String,
    pub class_name: String,
    pub subject: String,
    pub teacher: String,
    pub room: Room,
    pub modified_at: Option<NaiveDateTime>,
    pub modified_by: Option<String>,
    pub visible: bool,
}
