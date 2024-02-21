use crate::core::create_id;
use crate::core::User;
use crate::data::Database;
use anyhow::Result;
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

    pub async fn create<T: Database>(database: T, user: Option<&User>) -> Result<Self> {
        let session_id = create_id(30).await;
        let csrf_token = create_id(30).await;

        match user {
            Some(u) => {
                database
                    .create_session(Some(u), &session_id, &csrf_token)
                    .await?
            }
            None => {
                database
                    .create_session(None, &session_id, &csrf_token)
                    .await?
            }
        };

        match user {
            Some(u) => Ok(Session {
                id: session_id,
                csrf_token,
                user_id: Some(u.clone().id),
            }),
            None => Ok(Session {
                id: session_id,
                csrf_token,
                user_id: None,
            }),
        }
    }

    pub async fn delete<T: Database>(database: T, id: &str) -> Result<()> {
        database.delete_session(id).await?;
        Ok(())
    }
}
