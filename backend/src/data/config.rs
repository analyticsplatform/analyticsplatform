use crate::core::{Org, Session, User};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Database: Send + Sync + Clone + SessionStore + UserStore + 'static {}

#[async_trait]
pub trait UserStore: Send + Sync + Clone + 'static {
    async fn create_user(&self, user: &User) -> Result<()>;
    async fn get_user_by_email(&self, email: &str) -> Result<User>;
    async fn get_user_by_id(&self, id: &str) -> Result<User>;
    async fn create_org(&self, org: &Org) -> Result<()>;
    async fn get_org_by_id(&self, id: &str) -> Result<Org>;
    async fn delete_org(&self, id: &str) -> Result<()>;
}

#[async_trait]
pub trait SessionStore: Send + Sync + Clone + 'static {
    async fn get_session_by_id(&self, id: &str) -> Result<Session>;
    async fn create_session(
        &self,
        user: Option<&'life1 User>,
        session_id: &str,
        csrf_token: &str,
    ) -> Result<()>;
    async fn delete_session(&self, session_id: &str) -> Result<()>;
}
