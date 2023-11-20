use actix_web::http::StatusCode;
use actix_web::{delete, get, web, HttpResponse};
use anyhow::Result;
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::db_interactions;
use crate::types::Event;

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

async fn filter_events(query: EventQuery) -> Result<Vec<Event>> {
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

async fn filter_events_by_time(query: TimeQuery) -> Result<Vec<Event>> {
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
pub async fn show_events() -> HttpResponse {
    if let Ok(all_events) = db_interactions::get_all_events().await {
        HttpResponse::Ok()
            .status(StatusCode::OK)
            .json(web::Json(all_events))
    } else {
        HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Error with db.")
    }
}

#[get("/{id}")]
pub async fn event_detail(path: web::Path<(u64,)>) -> HttpResponse {
    if let Ok(event) = db_interactions::get_event(path.into_inner().0).await {
        match event {
            Some(e) => HttpResponse::Ok().status(StatusCode::OK).json(web::Json(e)),
            None => HttpResponse::Ok()
                .status(StatusCode::NOT_FOUND)
                .body("Event not found."),
        }
    } else {
        HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Error with db.")
    }
}

#[delete("/{id}")]
pub async fn delete_event(path: web::Path<(u64,)>) -> HttpResponse {
    if let Ok(event) = db_interactions::delete_event(path.into_inner().0).await {
        match event {
            Some(e) => HttpResponse::Ok().status(StatusCode::OK).json(web::Json(e)),
            None => HttpResponse::Ok()
                .status(StatusCode::NOT_FOUND)
                .body("Event not found."),
        }
    } else {
        HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Error with db.")
    }
}

#[get("/filter")]
pub async fn filter(query: web::Query<EventQuery>) -> HttpResponse {
    let filtered_events = filter_events(query.into_inner()).await;

    match filtered_events {
        Ok(events) => HttpResponse::Ok()
            .status(StatusCode::OK)
            .json(web::Json(events)),
        Err(_) => HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Error with db."),
    }
}

#[get("/bytime")]
pub async fn filter_by_time(query: web::Query<TimeQuery>) -> HttpResponse {
    let filtered_events = filter_events_by_time(query.into_inner()).await;

    match filtered_events {
        Ok(events) => HttpResponse::Ok()
            .status(StatusCode::OK)
            .json(web::Json(events)),
        Err(_) => HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Error with db."),
    }
}
