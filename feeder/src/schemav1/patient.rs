use clickhouse::Row;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

// TODO: Implement unwrap for the clickhouse library

#[derive(Debug)]
pub enum TimeResolution {
    Year,
    Month,
    Day,
}

impl Serialize for TimeResolution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(match self {
            TimeResolution::Year => "year",
            TimeResolution::Month => "month",
            TimeResolution::Day => "day",
        })
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Deceased {
    Unknown = 1,
    Alive = 2,
    Dead = 3,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AddressUse {
    Unknown = 0,
    Billing = 1,
    Home = 2,
    Old = 3,
    Temp = 4,
    Work = 5,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AddressType {
    Unknown = 0,
    Physical = 1,
    Postal = 2,
    Both = 3,
}

#[derive(Debug, Row, Serialize)]
pub struct AggregatePatient {
    pub id: String,
    pub name_given: String,
    pub name_family: String,
    #[serde(with = "clickhouse::serde::time::date32")]
    pub birth_time: time::Date,
    pub birth_time_resolution: TimeResolution,

    #[serde(with = "clickhouse::serde::time::datetime::option")]
    pub death_time: Option<time::OffsetDateTime>,
    pub deceased: Deceased,

    #[serde(rename = "addresses.use")]
    pub addresses_use: Vec<AddressUse>,
    #[serde(rename = "addresses.type")]
    pub addresses_type: Vec<AddressType>,
    #[serde(rename = "addresses.city")]
    pub addresses_city: Vec<String>,
    #[serde(rename = "addresses.line")]
    pub addresses_line: Vec<String>,
}
