use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Event {
    pub event_id: u64,
    pub from_datetime: NaiveDateTime,
    pub to_datetime: NaiveDateTime,
    pub department: String,
    pub class_name: String,
    pub subject: String,
    pub teacher: String,
    pub room: String,
    pub building: String,
    pub modified_at: Option<NaiveDateTime>,
    pub modified_by: Option<String>,
    pub visible: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub user_id: u64,
    pub username: String,
    pub passwordhash: u64,
    pub token: u64,
    pub is_admin: bool,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}
