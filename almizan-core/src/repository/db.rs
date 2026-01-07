use std::sync::Arc;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[derive(Clone)]
pub struct Database {
    pub client: Arc<Surreal<Client>>,
}

impl Database {
    pub async fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let db_url = std::env::var("DB_URL").unwrap_or("127.0.0.1:8000".to_string());
        let client = Surreal::new::<Ws>(db_url).await?;

        let db_user = std::env::var("DB_USER").unwrap_or("root".to_string());
        let db_pass = std::env::var("DB_PASS").unwrap_or("root".to_string());

        client
            .signin(Root {
                username: &db_user,
                password: &db_pass,
            })
            .await?;

        // Namespace configurable via environment (unified to 'almizan')
        let db_ns = std::env::var("DB_NS").unwrap_or("almizan".to_string());
        let db_name = std::env::var("DB_DB").unwrap_or("main".to_string());
        client.use_ns(&db_ns).use_db(&db_name).await?;

        let db = Self {
            client: Arc::new(client),
        };

        Ok(db)
    }
}
