use crate::core::{ConnectorDetails, Dataset, Org, Session, Team, User};
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
    async fn create_team(&self, org: &Team) -> Result<()>;
    async fn get_teams(&self) -> Result<Vec<Team>>;
    async fn get_team_by_id(&self, id: &str) -> Result<Team>;
    // Encrypt + Salt connection_string
    async fn create_connector(&self, conn: ConnectorDetails) -> Result<()>;
    async fn get_connectors(&self) -> Result<Vec<ConnectorDetails>>;
    async fn create_dataset(&self, dataset: Dataset) -> Result<()>;
    async fn get_datasets(&self) -> Result<Vec<Dataset>>;
}

#[async_trait]
pub trait SessionStore: Send + Sync + Clone + 'static {
    async fn get_session_by_id(&self, id: &str) -> Result<Session>;
    async fn create_session(&self, user: Option<&'life1 User>, session_id: &str) -> Result<()>;
    async fn delete_session(&self, session_id: &str) -> Result<()>;
}
