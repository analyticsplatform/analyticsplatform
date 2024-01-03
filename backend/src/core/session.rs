use crate::core::create_id;
use crate::core::User;
use crate::data::Database;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
}

impl Session {
    pub async fn from_id<T: Database>(database: T, id: &str) -> Result<Self> {
        Ok(database.get_session_by_id(id).await.unwrap())
    }

    pub async fn create<T: Database>(database: T, user: &User) -> Result<()> {
        let session_id = create_id(20).await;
        database.create_session(user, &session_id).await
    }
}
