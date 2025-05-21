mod utils;

use clickhouse::Client;
use feeder::{
    fhir_r4b_shemav1::{ConvertedBundle, convert_bundle, convert_patient},
    schemav1::db_ops::install_schema_v1,
};
use fhir_model::r4b::resources::{Bundle, Patient};
use utils::{connect_to_clickhouse_test_container, drop_db};

const BUNDLE_1: &str = include_str!("assets/bundle_1.json");

#[tokio::test]
async fn parse_and_insert() {
    let client = connect_to_clickhouse_test_container();

    let fhir_bundle = serde_json::from_str::<Bundle>(BUNDLE_1).unwrap();
    let converted_bundle = convert_bundle(&fhir_bundle).unwrap();
    dbg!(&converted_bundle);

    drop_db(&client, "attempt_1_1").await;
    install_schema_v1(&client, "attempt_1_1").await.unwrap();
}

