use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{delete, get, post, web, HttpResponse, ResponseError, Result};
use anyhow;
use chrono::NaiveDateTime;
use derive_more::Display;
use serde::Deserialize;

use crate::db_interactions;
use crate::model::event::*;

#[derive(Debug, Deserialize)]
pub struct TimeQuery {
    pub from_datetime: Option<NaiveDateTime>,
    pub to_datetime: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct EventQuery {
    pub department: Option<String>,
    pub class_name: Option<String>,
    pub subject: Option<String>,
    pub teacher: Option<String>,
    pub room: Option<String>,
    pub building: Option<String>,
    pub visible: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitEvent {
    pub from_datetime: NaiveDateTime,
    pub to_datetime: NaiveDateTime,
    pub department: String,
    pub class_name: String,
    pub subject: String,
    pub teacher: String,
    pub room: String,
    pub building: String,
    pub modified_by: String,
    pub visible: bool,
}

#[derive(Debug, Display)]
pub enum EventError {
    EventNotFound,
    EventUpdateFailure,
    EventCreationFailure,
    EventDBError,
}

impl ResponseError for EventError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            EventError::EventNotFound => StatusCode::NOT_FOUND,
            EventError::EventUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            EventError::EventCreationFailure => StatusCode::FAILED_DEPENDENCY,
            EventError::EventDBError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

async fn filter_events(query: EventQuery) -> anyhow::Result<Vec<Event>> {
    let mut events = db_interactions::get_all_events().await?;

    macro_rules! filter_field {
        ($field:ident) => {
            if let Some(value) = query.$field {
                events = events.into_iter().filter(|e| e.$field == value).collect();
            }
        };
    }

    filter_field!(department);
    filter_field!(class_name);
    filter_field!(subject);
    filter_field!(teacher);
    filter_field!(room);
    filter_field!(building);
    filter_field!(visible);

    Ok(events)
}

async fn filter_events_by_time(query: TimeQuery) -> anyhow::Result<Vec<Event>> {
    let mut events = db_interactions::get_all_events().await?;

    if let Some(from_datetime) = query.from_datetime {
        events = events
            .into_iter()
            .filter(|e| e.from_datetime >= from_datetime)
            .collect();
    }
    if let Some(to_datetime) = query.to_datetime {
        events = events
            .into_iter()
            .filter(|e| e.to_datetime <= to_datetime)
            .collect();
    }

    Ok(events)
}

#[get("")]
pub async fn show_events() -> Result<Json<Vec<Event>>, EventError> {
    match db_interactions::get_all_events().await {
        Ok(events) => Ok(Json(events)),
        Err(_) => Err(EventError::EventNotFound),
    }
}

#[get("/{id}")]
pub async fn event_detail(path: web::Path<(u64,)>) -> Result<Json<Event>, EventError> {
    if let Ok(event) = db_interactions::get_event(path.into_inner().0).await {
        match event {
            Some(e) => Ok(Json(e)),
            None => Err(EventError::EventNotFound),
        }
    } else {
        Err(EventError::EventDBError)
    }
}

#[delete("/{id}")]
pub async fn delete_event(path: web::Path<(u64,)>) -> Result<Json<Event>, EventError> {
    if let Ok(event) = db_interactions::delete_event(path.into_inner().0).await {
        match event {
            Some(e) => Ok(Json(e)),
            None => Err(EventError::EventNotFound),
        }
    } else {
        Err(EventError::EventDBError)
    }
}

#[get("/filter")]
pub async fn filter(query: web::Query<EventQuery>) -> Result<Json<Vec<Event>>, EventError> {
    let filtered_events = filter_events(query.into_inner()).await;

    match filtered_events {
        Ok(events) => Ok(Json(events)),
        Err(_) => Err(EventError::EventDBError),
    }
}

#[get("/bytime")]
pub async fn filter_by_time(query: web::Query<TimeQuery>) -> Result<Json<Vec<Event>>, EventError> {
    let filtered_events = filter_events_by_time(query.into_inner()).await;

    match filtered_events {
        Ok(events) => Ok(Json(events)),
        Err(_) => Err(EventError::EventDBError),
    }
}

#[post("/new")]
pub async fn create_new_event(request: Json<SubmitEvent>) -> Result<Json<Event>, EventError> {
    let new_event = Event::from_submit(request.into_inner());

    if let Ok(event) = db_interactions::create_event(new_event).await {
        match event {
            Some(e) => Ok(Json(e)),
            None => Err(EventError::EventCreationFailure),
        }
    } else {
        Err(EventError::EventDBError)
    }
}

#[post("/refresh_db")]
pub async fn update_db() -> Result<Json<DBUpdateData>, EventError> {
    let update_data = db_interactions::update_db().await;

    Ok(Json(update_data))
}
