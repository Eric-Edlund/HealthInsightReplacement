use clickhouse::Row;
use serde::Serialize;

// TODO: Implement unwrap for the clickhouse library

#[derive(Debug, Row, Serialize)]
pub struct AggregatePatient {
    pub name_given: String,
    pub name_family: String,
}
