#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Event {
    pub pxid: u64,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
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
