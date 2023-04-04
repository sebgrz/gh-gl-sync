use super::DB;
use tokio_postgres::Client;

mod embedded_migration {
    use refinery::embed_migrations;
    embed_migrations!("./db/migrations");
}

impl DB {
    pub async fn migrate(&mut self) {
        let client: &mut Client = &mut self.client;
        embedded_migration::migrations::runner()
            .run_async(client)
            .await
            .unwrap();
    }
}
