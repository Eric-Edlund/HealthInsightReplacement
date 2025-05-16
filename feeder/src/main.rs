mod fhir_r4b_shemav1;
mod schemav1;

use clickhouse::Client;
use fhir_model::r4b::resources::Patient;
use fhir_r4b_shemav1::convert_patient;

use kafka::consumer::Consumer;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), clickhouse::error::Error> {
    Consumer::from_hosts(vec!["".to_string()]).with_topic("".to_string());

    let mut clickhouse = Client::default()
        .with_url("http://localhost:8123")
        .with_user("eric")
        .with_password("1234");

    schemav1::db_ops::install_schema_v1(&clickhouse, "attempt_1_1").await?;

    // let fhir_patient = serde_json::from_str::<Patient>(PATIENT).unwrap();
    // let aggregate_patient = convert_patient(&fhir_patient).unwrap();
    // dbg!(&aggregate_patient);
    //
    // let mut insert = clickhouse.insert("attempt_1_1.AggregatePatient")?;
    // insert.write(&aggregate_patient).await?;
    // insert.end().await?;

    println!("Hello, world!");

    Ok(())
}
