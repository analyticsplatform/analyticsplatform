use crate::{core::create_id, data::Database, AppState};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dataset {
    pub id: String,
    pub name: String,
    pub provider: Option<String>,
    pub connector_id: String,
    pub path: String,
    pub description: String,
    pub schema: HashMap<String, String>,
    pub tags: Vec<String>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateDataset {
    pub name: String,
    pub provider: Option<String>,
    pub connector_id: String,
    pub path: String,
    pub description: String,
    pub tags: Vec<String>,
    pub metadata: Option<HashMap<String, String>>,
}

impl Dataset {
    pub async fn create<D: Database>(state: AppState<D>, payload: CreateDataset) -> Result<()> {
        let id = create_id(8).await;

        // Check if connector exists
        let connector = state
            .connections
            .get(&payload.connector_id)
            .context("Connector: Not Found")?;

        // TODO: Check is dataset (path) exists
        let data_info = connector.get_data_info(&payload.path.clone()).await?;

        state
            .db
            .create_dataset(Dataset {
                id,
                name: payload.name,
                provider: payload.provider,
                connector_id: payload.connector_id,
                path: payload.path,
                description: payload.description,
                schema: data_info.schema,
                tags: payload.tags,
                metadata: payload.metadata,
            })
            .await?;

        Ok(())
    }

    pub async fn get_all<D: Database>(database: D) -> Result<Vec<Dataset>> {
        let datasets = database.get_datasets().await?;
        Ok(datasets)
    }
}
