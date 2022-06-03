use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone, Debug)]
pub struct Environment {
    pool: PgPool,
}

impl juniper::Context for Environment {}

impl Environment {
    pub async fn new(url: &str) -> Result<Self> {
        let db_pool = PgPoolOptions::new().max_connections(5).connect(url).await?;

        Ok(Self { pool: db_pool })
    }
}
