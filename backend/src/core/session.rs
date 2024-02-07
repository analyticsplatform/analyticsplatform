use crate::core::create_id;
use crate::core::User;
use crate::data::Database;
use anyhow::{anyhow, Result};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Session {
    pub id: String,
    pub csrf_token: String,
    pub user_id: Option<String>,
}

impl Session {
    // TODO: dead code
    pub async fn from_id<T: Database>(database: T, id: &str) -> Result<Self> {
        database.get_session_by_id(id).await
    }

    pub async fn create<T: Database>(database: T, user: &User) -> Result<Self> {
        let session_id = create_id(30).await;
        let csrf_token = create_id(30).await;
        let db_resp = database
            .create_session(user, &session_id, &csrf_token)
            .await;

        match db_resp {
            Ok(_) => Ok(Session {
                id: session_id,
                csrf_token,
                user_id: Some(user.clone().id),
            }),
            Err(_) => Err(anyhow!("failed to create session")),
        }
    }

    pub async fn delete<T: Database>(database: T, id: &str) -> Result<()> {
        database.delete_session(id).await?;
        Ok(())
    }
}
