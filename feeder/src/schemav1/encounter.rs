use clickhouse::Row;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EncounterStatus {
    Arrived = 0,
    Cancelled = 1,
    EnteredInError = 2,
    Finished = 3,
    InProgress = 4,
    Onleave = 5,
    Planned = 6,
    Triaged = 7,
    Unknown = 8,
}

#[derive(Debug, Row, Serialize)]
pub struct Encounter {
    pub id: String,
    pub status: EncounterStatus,
    /// Patient id
    pub subject: String,

    #[serde(with = "clickhouse::serde::time::datetime")]
    pub period_start: time::OffsetDateTime,
    #[serde(with = "clickhouse::serde::time::datetime")]
    pub period_end: time::OffsetDateTime,

    pub class_code: String,
    pub class_description: String,
    pub class_system: String,

    // pub type_code: String,
    // pub type_description: String,
    // pub type_system: String,

}
