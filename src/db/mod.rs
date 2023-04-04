use crate::config::Database;
use tokio_postgres::{Client, NoTls};

pub mod migration;

pub struct DB {
    client: Client,
}

impl DB {
    pub async fn new(config: &Database) -> DB {
        let (client, connection) = tokio_postgres::connect(
            format!(
                "host={} user={} password={} dbname={} connect_timeout=10",
                config.host, config.username, config.password, config.db
            )
            .as_str(),
            NoTls,
        )
        .await
        .unwrap();

        tokio::spawn(async {
            connection.await.unwrap();
            println!("end db connection");
        });

        DB {
            client
        }
    }
}
