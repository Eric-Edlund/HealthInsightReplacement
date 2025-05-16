use feeder::fhir_r4b_shemav1::convert_encounter;
use fhir_model::r4b::resources::Encounter;

const ENCOUNTER_1: &str = include_str!("assets/encounter_1.json");

#[tokio::test]
async fn parse_1() {
    // let mock_db = clickhouse::test::Mock::new();
    // let mut client = clickhouse::Client::default().with_url(mock_db.url());

    let fhir_patient = serde_json::from_str::<Encounter>(ENCOUNTER_1).unwrap();
    let aggregate_patient = convert_encounter(&fhir_patient).unwrap();
    dbg!(&aggregate_patient);
}
