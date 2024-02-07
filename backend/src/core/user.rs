use crate::core::create_id;
use crate::data::Database;
use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub r#type: String,
    pub is_active: bool,
    pub hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub r#type: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Email {
    pub email: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub r#type: String,
    pub is_active: bool,
}

impl User {
    pub async fn create<T: Database>(database: T, user: &CreateUser) -> Result<()> {
        let user_id = create_id(10).await;
        let new_user = User::from_create_user(user, &user_id, true);
        database.create_user(&new_user).await
    }

    pub async fn from_id<T: Database>(database: T, id: &str) -> Result<User> {
        database.get_user_by_id(id).await
    }

    pub async fn from_email<T: Database>(database: T, email: &str) -> Result<User> {
        database.get_user_by_email(email).await
    }

    fn from_create_user(create_user: &CreateUser, id: &str, is_active: bool) -> User {
        // Generate password hash
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(create_user.password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        User {
            id: id.to_string(),
            email: create_user.email.to_string(),
            first_name: create_user.first_name.to_string(),
            last_name: create_user.last_name.to_string(),
            r#type: create_user.r#type.to_string(),
            is_active,
            hash: password_hash,
        }
    }
}

impl From<User> for Profile {
    fn from(value: User) -> Self {
        Profile {
            id: value.id,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            r#type: value.r#type,
            is_active: value.is_active,
        }
    }
}
