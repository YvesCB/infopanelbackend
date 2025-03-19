use axum::{
    extract::{Query, State},
    routing::post,
    Json, Router,
};
use serde::Deserialize;

use crate::{
    ctx::Ctx,
    model::room::{Room, RoomForCreate, RoomModelController},
    Result,
};

pub fn routes(room_mc: RoomModelController) -> Router {
    Router::new()
        .route("/rooms", post(create_room).get(list_rooms))
        .with_state(room_mc)
}

async fn create_room(
    ctx: Ctx,
    State(room_mc): State<RoomModelController>,
    Json(room_fc): Json<RoomForCreate>,
) -> Result<Json<Room>> {
    Ok(Json(room_mc.create(room_fc, ctx.user_id()).await?))
}

#[allow(unused)]
#[derive(Deserialize)]
struct RoomParams {
    location: Option<String>,
    roomname: Option<String>,
    capacity: Option<u32>,
}

async fn list_rooms(
    _ctx: Ctx,
    State(room_mc): State<RoomModelController>,
    Query(params): Query<RoomParams>,
) -> Result<Json<Vec<Room>>> {
    Ok(Json(room_mc.get_all().await?))
}
