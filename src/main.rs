#[allow(unused)]
pub use self::error::{Error, Result};

use std::net::SocketAddr;

use ::log::{error, info};
use axum::{
    http::{Method, Uri},
    middleware,
    response::{IntoResponse, Response},
    Json, Router,
};
use ctx::Ctx;
use log::log_request;
use model::{event::EventModelController, room::RoomModelController};
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize the logging
    log4rs::init_file("log_config.yml", Default::default()).unwrap();
    // initialize ModelController
    let event_mc = EventModelController::new().await?;
    let room_mc = RoomModelController::new().await?;

    let routes_apis = web::routes_events::routes(event_mc.clone())
        .merge(web::routes_rooms::routes(room_mc.clone()))
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let app = Router::new()
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            event_mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new());

    // construct the address and the listener. Axum uses tokio's own tcp listener.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8088));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("->> LISTENING on {addr}\n");

    // we pass the listener and any struct that can be a service.
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn main_response_mapper(
    ctx: Result<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    //info!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the error if it is there
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- if client error, build new response
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_body_error = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
            }
            });

            error!("client_error_body: {client_body_error}");

            // Build the new response
            (*status_code, Json(client_body_error)).into_response()
        });

    let client_error = client_status_error.unzip().1;
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    error_response.unwrap_or(res)
}
