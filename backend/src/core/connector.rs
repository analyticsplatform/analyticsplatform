use anyhow::Result;
use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::Duration;

use std::fmt::Debug;

//use crate::core::common::create_id;

#[async_trait]
pub trait Connector: Send + Sync + Debug {
    // Define common methods for connections here
    async fn get_datasets(&self) -> Result<HashMap<String, Vec<String>>>;
}

pub async fn create_connections_from_env() -> Arc<HashMap<String, Box<dyn Connector>>> {
    let mut connections = HashMap::new();

    // Iterate over all environment variables
    for (key, value) in env::vars() {
        // Check if the key starts with "AP_CONNECTION"
        if key.starts_with("AP_CONNECTION") && value.starts_with("postgres://") {
            // Attempt to create a PgPool from the value

            let pool = PgPoolOptions::new()
                .max_connections(2)
                .acquire_timeout(Duration::new(5, 0))
                .connect(&value)
                .await
                .unwrap();

            //let uuid = create_id(8).await;
            let uuid = String::from("ABC123");
            connections.insert(uuid, Box::new(pool) as Box<dyn Connector>);
        }
    }

    Arc::new(connections)
}

#[async_trait]
impl Connector for Pool<Postgres> {
    async fn get_datasets(&self) -> Result<HashMap<String, Vec<String>>> {
        let rows = sqlx::query(
            "SELECT schemaname, tablename FROM pg_tables WHERE schemaname NOT IN ('pg_catalog', 'information_schema')"
        )
        .fetch_all(self)
        .await?;

        let mut tables_by_schema: HashMap<String, Vec<String>> = HashMap::new();

        for row in rows {
            let schemaname: String = row.try_get("schemaname")?;
            let tablename: String = row.try_get("tablename")?;

            tables_by_schema
                .entry(schemaname)
                .or_insert_with(Vec::new)
                .push(tablename);
        }

        Ok(tables_by_schema)
    }
}
