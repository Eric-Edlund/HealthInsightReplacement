use feeder::fhir_r4b_shemav1::convert_patient;
use fhir_model::r4b::resources::Patient;

const PATIENT_1: &str = include_str!("assets/patient_1.json");

#[tokio::test]
async fn parse_patient_1() {
    // let mock_db = clickhouse::test::Mock::new();
    // let mut client = clickhouse::Client::default().with_url(mock_db.url());

    let fhir_patient = serde_json::from_str::<Patient>(PATIENT_1).unwrap();
    let aggregate_patient = convert_patient(&fhir_patient).unwrap();
    dbg!(&aggregate_patient);
}
