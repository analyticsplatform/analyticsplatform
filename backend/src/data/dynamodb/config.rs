use crate::core::{CreateUser, Email, Session, User};
use crate::data::{Database, SessionStore, UserStore};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue as AV;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct Dynamodb {
    pub client: Client,
    pub table_name: String,
}

impl Database for Dynamodb {}

impl Dynamodb {
    pub async fn new(local: bool, table_name: &str) -> Result<Self> {
        let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");

        // Set endpoint url to localhost to run locally
        let config = match local {
            true => {
                let defaults = aws_config::defaults(BehaviorVersion::latest())
                    .region(region_provider)
                    .load()
                    .await;
                aws_sdk_dynamodb::config::Builder::from(&defaults)
                    .endpoint_url("http://localhost:8000")
                    .build()
            }
            false => {
                let defaults = aws_config::defaults(BehaviorVersion::latest())
                    .region(region_provider)
                    .load()
                    .await;
                aws_sdk_dynamodb::config::Builder::from(&defaults).build()
            }
        };

        let client = Client::from_conf(config);

        // Check if table exists
        let resp = &client.list_tables().send().await.unwrap();
        let tables = resp.table_names();

        if !tables.contains(&table_name.to_string()) {
            error!("table does not exist");
            return Err(anyhow!(""));
        }

        let dynamodb = Dynamodb {
            client: client.clone(),
            table_name: table_name.into(),
        };

        let admin_user = User::from_email(dynamodb.clone(), "test@example.com").await;

        match admin_user {
            Ok(_) => {
                info!("db init: admin user exists.");
            }
            Err(_) => {
                info!("db init: creating admin user.");
                let admin_user = CreateUser {
                    email: String::from("test@example.com"),
                    first_name: String::from("Admin"),
                    last_name: String::from("Istrator"),
                    r#type: String::from("superadmin"),
                };

                let _admin_user = User::create(dynamodb.clone(), &admin_user).await;
            }
        }

        Ok(dynamodb)
    }
}

fn split_at_hash(input: &str) -> &str {
    input.split_once('#').unwrap().1
}

impl From<HashMap<String, AV>> for User {
    fn from(value: HashMap<String, AV>) -> Self {
        let user = User {
            id: split_at_hash(value.get("PK").unwrap().as_s().unwrap()).to_string(),
            email: split_at_hash(value.get("GSI1PK").unwrap().as_s().unwrap()).to_string(),
            first_name: value.get("first_name").unwrap().as_s().unwrap().to_string(),
            last_name: value.get("last_name").unwrap().as_s().unwrap().to_string(),
            is_active: *value.get("is_active").unwrap().as_bool().unwrap(),
            r#type: value.get("user_type").unwrap().as_s().unwrap().to_string(),
        };
        user
    }
}

impl From<HashMap<String, AV>> for Email {
    fn from(value: HashMap<String, AV>) -> Self {
        Email {
            email: split_at_hash(value.get("PK").unwrap().as_s().unwrap()).to_string(),
            user_id: split_at_hash(value.get("GSI1PK").unwrap().as_s().unwrap()).to_string(),
        }
    }
}

impl From<HashMap<String, AV>> for Session {
    fn from(value: HashMap<String, AV>) -> Self {
        Session {
            id: split_at_hash(value.get("PK").unwrap().as_s().unwrap()).to_string(),
            user_id: split_at_hash(value.get("GSI1PK").unwrap().as_s().unwrap()).to_string(),
        }
    }
}

#[async_trait]
impl UserStore for Dynamodb {
    async fn create_user(&self, user: &User) -> Result<()> {
        // Create the USER item to insert
        let mut item = std::collections::HashMap::new();
        let key = format!("{}{}", "USER#", user.id);
        let email = format!("{}{}", "EMAIL#", user.email);
        let r#type = format!("{}{}", "USERTYPE#", user.r#type);

        item.insert(String::from("PK"), AV::S(key.clone()));
        item.insert(String::from("SK"), AV::S(key.clone()));
        item.insert(String::from("GSI1PK"), AV::S(email.clone()));
        item.insert(String::from("GSI1SK"), AV::S(email.clone()));
        item.insert(String::from("GSI2PK"), AV::S(r#type.clone()));
        item.insert(String::from("GSI2SK"), AV::S(r#type));
        item.insert(String::from("first_name"), AV::S(user.first_name.clone()));
        item.insert(String::from("last_name"), AV::S(user.last_name.clone()));
        item.insert(String::from("user_type"), AV::S(user.r#type.clone()));
        item.insert(String::from("is_active"), AV::Bool(user.is_active));

        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await?;

        // Create the EMAIL item to insert
        let mut email_item = std::collections::HashMap::new();
        email_item.insert(String::from("PK"), AV::S(email.clone()));
        email_item.insert(String::from("SK"), AV::S(email));
        email_item.insert(String::from("GSI1PK"), AV::S(key.clone()));
        email_item.insert(String::from("GSI1SK"), AV::S(key.clone()));

        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(email_item))
            .send()
            .await?;

        Ok(())
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let email_key = format!("EMAIL#{email}");
        match self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("PK", AV::S(email_key.clone()))
            .key("SK", AV::S(email_key))
            .send()
            .await
        {
            Ok(response) => {
                let response_item = response.clone().item;

                if let Some(email_item) = response_item {
                    let email_record: Email = email_item.into();
                    UserStore::get_user_by_id(self, &email_record.user_id).await
                } else {
                    Err(anyhow!("email not found"))
                }
            }
            Err(_e) => Err(anyhow!("email not found")),
        }
    }

    async fn get_user_by_id(&self, id: &str) -> Result<User> {
        let key = format!("USER#{id}");
        match self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("PK", AV::S(key.clone()))
            .key("SK", AV::S(key.into()))
            .send()
            .await
        {
            Ok(response) => Ok(response.item.unwrap().into()),
            Err(_e) => Err(anyhow!("user not found")),
        }
    }
}

#[async_trait]
impl SessionStore for Dynamodb {
    async fn get_session_by_id(&self, id: &str) -> Result<Session> {
        let key = format!("SESSION#{id}");
        match self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key("PK", AV::S(key.clone()))
            .key("SK", AV::S(key))
            .send()
            .await
        {
            Ok(response) => Ok(response.item.unwrap().into()),
            Err(_e) => Err(anyhow!("session not found")),
        }
    }

    async fn create_session(&self, user: &User, session_id: &str) -> Result<()> {
        // Create the item to insert
        let mut item = std::collections::HashMap::new();
        let key = format!("{}{}", "SESSION#", user.id);

        item.insert(String::from("PK"), AV::S(key.clone()));
        item.insert(String::from("SK"), AV::S(key));
        item.insert(String::from("GSI1PK"), AV::S(session_id.to_string()));
        item.insert(String::from("GSI1SK"), AV::S(session_id.to_string()));

        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await?;
        Ok(())
    }
}
