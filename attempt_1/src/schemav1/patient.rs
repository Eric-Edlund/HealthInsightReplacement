use clickhouse::Row;
use serde::Serialize;

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

#[derive(Debug, Row, Serialize)]
pub struct AggregatePatient {
    pub name_given: String,
    pub name_family: String,
    #[serde(with = "clickhouse::serde::time::date32")]
    pub birth_time: time::Date,
    pub birth_time_resolution: TimeResolution,

    #[serde(with = "clickhouse::serde::time::datetime")]
    pub death_time: time::OffsetDateTime,
    // pub deceased: bool,
}
