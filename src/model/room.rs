use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Room {
    location: String,
    roomname: String,
    capacity: u32,
}
