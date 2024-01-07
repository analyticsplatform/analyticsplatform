use crate::core::create_id;
use crate::data::Database;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub r#type: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Email {
    pub email: String,
    pub user_id: String,
}

impl User {
    pub async fn create<T: Database>(database: T, user: &CreateUser) -> Result<()> {
        let user_id = create_id(10).await;
        let new_user = User::from_create_user(user, &user_id, true);
        database.create_user(&new_user).await
    }

    pub async fn from_email<T: Database>(database: T, email: &str) -> Result<User> {
        database.get_user_by_email(email).await
    }

    fn from_create_user(create_user: &CreateUser, id: &str, is_active: bool) -> User {
        User {
            id: id.to_string(),
            email: create_user.email.to_string(),
            first_name: create_user.first_name.to_string(),
            last_name: create_user.last_name.to_string(),
            r#type: create_user.r#type.to_string(),
            is_active,
        }
    }
}
