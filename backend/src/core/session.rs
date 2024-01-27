use crate::core::create_id;
use crate::core::User;
use crate::data::Database;
use anyhow::{anyhow, Result};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
}

impl Session {
    // TODO: dead code
    pub async fn _from_id<T: Database>(database: T, id: &str) -> Result<Self> {
        Ok(database.get_session_by_id(id).await.unwrap())
    }

    pub async fn create<T: Database>(database: T, user: &User) -> Result<Self> {
        let session_id = create_id(30).await;
        let db_resp = database.create_session(user, &session_id).await;

        match db_resp {
            Ok(_) => Ok(Session {
                id: session_id,
                user_id: user.clone().id,
            }),
            Err(_) => Err(anyhow!("failed to create session")),
        }
    }
}
