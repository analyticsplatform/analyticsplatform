use crate::core::common::create_id;
use crate::data::Database;
use anyhow::{anyhow, Result};
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataInfo {
    pub path: String,
    pub schema: HashMap<String, String>,
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
            connectors.insert(connector_detail.id, connector);
        }
        Ok(connectors)
    }

    pub async fn get_available_datasets(&self) -> Result<Vec<String>> {
        match self {
            Connector::Postgres(c) => c.get_available_datasets().await,
        }
    }

    pub async fn get_data_info(&self, path: &str) -> Result<DataInfo> {
        match self {
            Connector::Postgres(c) => c.get_data_info(path).await,
        }
    }
}

#[async_trait]
pub trait ConnectorTrait: Send + Sync + Debug {
    async fn create_record<D: Database>(database: D, conn: CreateConnector) -> Result<()>;
    async fn get_available_datasets(&self) -> Result<Vec<String>>;
    async fn get_data_info(&self, path: &str) -> Result<DataInfo>;
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

    async fn get_available_datasets(&self) -> Result<Vec<String>> {
        let rows = sqlx::query(
        "SELECT schemaname, tablename FROM pg_tables WHERE schemaname NOT IN ('pg_catalog', 'information_schema')"
    )
    .fetch_all(&self.pool)
    .await?;

        let mut datasets: Vec<String> = Vec::new();

        for row in rows {
            let schemaname: String = row.try_get("schemaname")?;
            let tablename: String = row.try_get("tablename")?;
            let dataset = format!("{}.{}", schemaname, tablename);
            datasets.push(dataset);
        }

        Ok(datasets)
    }

    async fn get_data_info(&self, path: &str) -> Result<DataInfo> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.len() != 2 {
            return Err(anyhow!(
                "Invalid path format. Expected: 'schema_name.table_name'"
            ));
        }
        let schema_name = parts[0];
        let table_name = parts[1];

        let query = format!(
            "SELECT column_name, data_type 
         FROM information_schema.columns
         WHERE table_schema = $1 AND table_name = $2"
        );

        let rows = sqlx::query(&query)
            .bind(schema_name)
            .bind(table_name)
            .fetch_all(&self.pool)
            .await?;

        if rows.is_empty() {
            return Err(anyhow!(
                "Table '{}.{}' does not exist",
                schema_name,
                table_name
            ));
        }

        let schema = rows
            .into_iter()
            .map(|row| {
                let column_name: String = row.get("column_name");
                let data_type: String = row.get("data_type");
                (column_name, data_type)
            })
            .collect();

        let data_info = DataInfo {
            path: format!("{}.{}", schema_name, table_name),
            schema,
        };

        Ok(data_info)
    }
}
