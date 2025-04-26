mod ccda;

use clickhouse::Client;

fn main() {
    let client = Client::default()
        .with_url("http://localhost:8123")
        .with_user("name")
        .with_password("123")
        .with_database("test");
}
