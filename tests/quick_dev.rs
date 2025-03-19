#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8088")?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    let req_create_room = hc.do_post(
        "/api/rooms",
        json!({
            "location": "Other Location",
            "roomname": "Room 201",
            "capacity": 25,
        }),
    );
    req_create_room.await?.print().await?;

    let req_create_event = hc.do_post(
        "/api/events",
        json!({
            "pxid": 123,
            "start": "2025-03-18T18:37:41",
            "end": "2025-03-18T19:37:41",
            "department": "JME",
            "classname": "ABC",
            "subject": "TEST",
            "teacher": "Max Muster",
            "room": "rooms:d4my4knnu9z506tivir8",
            "visible": true,
        }),
    );
    req_create_event.await?.print().await?;

    //hc.do_get("/api/events").await?.print().await?;

    Ok(())
}
