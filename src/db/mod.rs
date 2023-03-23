use tokio_postgres::NoTls;

mod embedded_migration {
    use refinery::embed_migrations;
    embed_migrations!("./db/migrations");
}

pub async fn migrate() {
    let (mut client, connection) = tokio_postgres::connect(
        "host=localhost user=user password=password dbname=gh-gl-sync connect_timeout=10",
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
