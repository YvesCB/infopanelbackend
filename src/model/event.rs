use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::event::SubmitEvent;
// use surrealdb::sql::Thing;

#[derive(Debug, Serialize)]
pub struct DBUpdateData {
    pub deleted_events: usize,
    pub answer_string: String,
    pub sucess: bool,
}

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

impl Event {
    pub fn from_submit(submit: SubmitEvent) -> Event {
        let (u64uuid, _) = Uuid::new_v4().as_u64_pair();
        let u64uuid = u64uuid / 2;
        Event {
            event_id: u64uuid,
            from_datetime: submit.from_datetime,
            to_datetime: submit.to_datetime,
            department: submit.department,
            class_name: submit.class_name,
            subject: submit.subject,
            teacher: submit.teacher,
            room: submit.room,
            building: submit.building,
            modified_at: Some(Local::now().naive_local()),
            modified_by: Some(submit.modified_by),
            visible: submit.visible,
        }
    }
}

// #[derive(Debug, Deserialize)]
// pub struct Record {
//     #[allow(dead_code)]
//     id: u64,
// }
