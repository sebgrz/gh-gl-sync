use tokio_postgres::NoTls;

use crate::config::Database;

mod embedded_migration {
    use refinery::embed_migrations;
    embed_migrations!("./db/migrations");
}

pub async fn migrate(config: &Database) {
    let (mut client, connection) = tokio_postgres::connect(
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
        println!("end migration connection");
    });

    embedded_migration::migrations::runner()
        .run_async(&mut client)
        .await
        .unwrap();
}
