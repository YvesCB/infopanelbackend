use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub user_id: u64,
    pub username: String,
    pub passwordhash: u64,
    pub token: u64,
    pub is_admin: bool,
}
