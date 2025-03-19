use axum::{
    extract::{Query, State},
    routing::post,
    Json, Router,
};
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::{
    ctx::Ctx,
    model::event::{Event, EventForCreate, EventModelController},
    Result,
};

pub fn routes(event_mc: EventModelController) -> Router {
    Router::new()
        .route("/events", post(create_event).get(list_events))
        .with_state(event_mc)
}

async fn create_event(
    ctx: Ctx,
    State(event_mc): State<EventModelController>,
    Json(event_fc): Json<EventForCreate>,
) -> Result<Json<Event>> {
    Ok(Json(event_mc.create(event_fc, ctx.user_id()).await?))
}

#[allow(unused)]
#[derive(Deserialize)]
struct EventParams {
    from: Option<NaiveDateTime>,
    to: Option<NaiveDateTime>,
    department: Option<String>,
    classname: Option<String>,
    subject: Option<String>,
    teacher: Option<String>,
}

async fn list_events(
    _ctx: Ctx,
    State(event_mc): State<EventModelController>,
    Query(params): Query<EventParams>,
) -> Result<Json<Vec<Event>>> {
    Ok(Json(event_mc.get_all().await?))
}
