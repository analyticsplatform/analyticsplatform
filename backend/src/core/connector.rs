use crate::core::common::create_id;
use crate::data::Database;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct PostgresConnector {
    pub pool: PgPool,
}

#[derive(Debug, Clone)]
pub enum Connector {
    Postgres(PostgresConnector),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ConnectorType {
    Postgres,
}

impl fmt::Display for ConnectorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConnectorType::Postgres => write!(f, "postgres"),
        }
    }
}

impl From<String> for ConnectorType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "postgres" => ConnectorType::Postgres,
            _ => panic!("Invalid connector type: {}", s),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateConnector {
    pub name: String,
    pub r#type: ConnectorType,
    pub connection_string: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConnectorDetails {
    pub id: String,
    pub name: String,
    pub r#type: ConnectorType,
    pub connection_string: String,
}

impl ConnectorDetails {
    pub async fn get_connector_details<D: Database>(
        database: D,
        hide_connection_string: bool,
    ) -> Result<Vec<Self>> {
        let mut connector_details = database.get_connectors().await?;

        if hide_connection_string {
            for connector in connector_details.iter_mut() {
                connector.connection_string = "***".to_string();
            }
        }

        Ok(connector_details)
    }
}

impl Connector {
    pub async fn create_connectors<D: Database>(database: D) -> Result<HashMap<String, Connector>> {
        //
        let connector_details = ConnectorDetails::get_connector_details(database, false).await?;

        let mut connectors = HashMap::new();
        for connector_detail in connector_details {
            let pool = PgPoolOptions::new()
                .max_connections(2)
                .acquire_timeout(Duration::new(5, 0))
                .connect(&connector_detail.connection_string)
                .await
                .unwrap();
            let connector = Connector::Postgres(PostgresConnector { pool });
            connectors.insert(connector_detail.name, connector);
        }
        Ok(connectors)
    }

    pub async fn get_datasets(&self) -> Result<HashMap<String, Vec<String>>> {
        match self {
            Connector::Postgres(c) => c.get_datasets().await, // Add more match arms for other variants of Connector if needed
        }
    }
}

#[async_trait]
pub trait ConnectorTrait: Send + Sync + Debug {
    async fn create_record<D: Database>(database: D, conn: CreateConnector) -> Result<()>;
    async fn get_datasets(&self) -> Result<HashMap<String, Vec<String>>>;
}

#[async_trait]
impl ConnectorTrait for PostgresConnector {
    async fn create_record<D: Database>(database: D, conn: CreateConnector) -> Result<()> {
        let id = create_id(8).await;
        let connector_details = ConnectorDetails {
            id,
            name: conn.name,
            r#type: conn.r#type,
            connection_string: conn.connection_string,
        };
        database.create_connector(connector_details).await?;
        Ok(())
    }

    async fn get_datasets(&self) -> Result<HashMap<String, Vec<String>>> {
        let rows = sqlx::query(
            "SELECT schemaname, tablename FROM pg_tables WHERE schemaname NOT IN ('pg_catalog', 'information_schema')"
        )
        .fetch_all(&self.pool)
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
