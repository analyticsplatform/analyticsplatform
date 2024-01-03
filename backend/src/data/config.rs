use crate::core::{Session, User};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Database: Send + Sync + Clone + SessionStore + UserStore {}

#[async_trait]
pub trait UserStore: Send + Sync {
    async fn create_user(&self, user: &User) -> Result<()>;
    async fn get_user_by_email(&self, email: &str) -> Result<User>;
    async fn get_user_by_id(&self, id: &str) -> Result<User>;
}

#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn get_session_by_id(&self, id: &str) -> Result<Session>;
    async fn create_session(&self, user: &User, session_id: &str) -> Result<()>;
}
