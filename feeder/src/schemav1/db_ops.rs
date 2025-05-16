use clickhouse::Client;

const MAKE_PATIENT_TABLE: &str = include_str!("sql/make_patient.sql");

pub async fn install_schema_v1(
    client: &Client,
    db_name: &str,
) -> Result<(), clickhouse::error::Error> {
    let mut client = client.clone();
    client
        .query(&format!("CREATE DATABASE IF NOT EXISTS {}", db_name))
        .execute()
        .await?;
    client = client.with_database(db_name);
    client.query(MAKE_PATIENT_TABLE).execute().await.unwrap();

    Ok(())
}
