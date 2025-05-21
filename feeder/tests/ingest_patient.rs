mod utils;

use feeder::{fhir_r4b_shemav1::convert_patient, schemav1::db_ops::install_schema_v1};
use fhir_model::r4b::resources::Patient;
use utils::{drop_db, connect_to_clickhouse_test_container};

const PATIENT_1: &str = include_str!("assets/patient_1.json");

#[tokio::test]
async fn parse_and_insert() {
    let client = connect_to_clickhouse_test_container();

    let fhir_patient = serde_json::from_str::<Patient>(PATIENT_1).unwrap();
    let aggregate_patient = convert_patient(&fhir_patient).unwrap();
    dbg!(&aggregate_patient);


    drop_db(&client, "attempt_1_1").await;
    install_schema_v1(&client, "attempt_1_1").await.unwrap();

    let mut insert = client.insert("attempt_1_1.AggregatePatient").unwrap();
    insert.write(&aggregate_patient).await.unwrap();
    insert.end().await.unwrap();
}
