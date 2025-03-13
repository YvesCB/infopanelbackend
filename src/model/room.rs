use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Room {
    #[serde(skip_serializing_if = "Option::is_none")] // Don't include `id` when creating
    pub id: Option<RecordId>,
    pub location: String,
    pub roomname: String,
    pub capacity: u32,
}
