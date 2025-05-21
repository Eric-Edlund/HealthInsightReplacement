use clickhouse::Client;

pub fn connect_to_clickhouse_test_container() -> Client {
    Client::default()
        .with_url("http://localhost:8123")
        .with_user("eric")
        .with_password("1234")
}

pub async fn drop_db(client: &Client, db_name: &str) {
    client.query(&format!("DROP DATABASE IF EXISTS {}", db_name)).execute().await.unwrap()
}
