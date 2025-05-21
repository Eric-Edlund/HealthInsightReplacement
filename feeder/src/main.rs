mod fhir_r4b_shemav1;
mod schemav1;

use std::{collections::HashMap, sync::Mutex};

use clickhouse::Client;
use futures::StreamExt;
use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication}, config::FromClientConfig, consumer::{MessageStream, StreamConsumer}, message::BorrowedMessage, ClientConfig, Message
};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use tokio::sync::Notify;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), clickhouse::error::Error> {
    let mut kafka_client_config = ClientConfig::new();
    kafka_client_config.set("metadata.broker.list", "localhost:9092");
    kafka_client_config.set("group.id", "test-group");

    let ingress_topic = NewTopic::new("test_bundles", 1, TopicReplication::Fixed(1));
    let kafka_opts = AdminOptions::new();
    let admin = AdminClient::from_config(&kafka_client_config).unwrap();
    admin
        .create_topics([&ingress_topic], &kafka_opts)
        .await
        .unwrap();

    let bundle_ingress_consumer =
        StreamConsumer::from_config(&kafka_client_config).unwrap();

    bundle_ingress_consumer.commit_message(msg, CommitMode::Async)

    let mut clickhouse = Client::default()
        .with_url("http://localhost:8123")
        .with_user("eric")
        .with_password("1234");

    schemav1::db_ops::install_schema_v1(&clickhouse, "attempt_1_1").await?;

    let buffer = Mutex::new(AllocRingBuffer::<(BorrowedMessage, bool)>::new(10));
    let commit_advanced = Notify::new();
    let bundle_read = Notify::new();

    read_kafka(&bundle_ingress_consumer, &buffer, commit_advanced);

    println!("Hello, world!");

    loop {
        while buffer.is_full() {
            if buffer.back().unwrap().1 {
                let _ = buffer.dequeue();
                continue
            }
            commit_advanced.notified().await

            // If the lowest message id in the committed_messages list is 
            // adjacent to the Consumer's leader, commit messages up to 
            // the first non-consecutive commit, removing them from the commmit
            // list. Then these messages from the ring buffer so that we can
            // read more.
        }

        let msg = bundle_ingress_consumer.recv().await.unwrap();
        buffer.push((msg, false));
    }

    // let fhir_patient = serde_json::from_str::<Patient>(PATIENT).unwrap();
    // let aggregate_patient = convert_patient(&fhir_patient).unwrap();
    // dbg!(&aggregate_patient);
    //
    // let mut insert = clickhouse.insert("attempt_1_1.AggregatePatient")?;
    // insert.write(&aggregate_patient).await?;
    // insert.end().await?;
}

async fn read_kafka(stream: &Mutex<StreamConsumer>) {

}

// async fn insert_bundle(
//     client: &Client,
//     db_name: &str,
//     bundle: ConvertedBundle,
// ) -> Result<(), clickhouse::error::Error> {
//     let inserter = client.inserter("attempt_1_1.AggregatePatient").unwrap();
//     let mut insert = client.insert("attempt_1_1.AggregatePatient").unwrap();
//     for res in &bundle.resources {
//         insert.write(res).await.unwrap();
//     }
//     insert.end().await.unwrap();
// }
