use crate::core::common::create_id;
use crate::core::{ConnectorDetails, ConnectorTrait, CreateConnector, DataInfo};
use crate::data::Database;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::{PgPool, Row};

#[derive(Debug, Clone)]
pub struct PostgresConnector {
    pub pool: PgPool,
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

        let query = "SELECT column_name, data_type 
         FROM information_schema.columns
         WHERE table_schema = $1 AND table_name = $2"
            .to_string();

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
