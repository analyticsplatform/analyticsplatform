use crate::core::{ConnectorDetails, Dataset, Email, Org, Session, Team, User};
use aws_sdk_dynamodb::types::AttributeValue as AV;
use std::collections::HashMap;

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
            hash: value.get("hash").unwrap().as_s().unwrap().to_string(),
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
        // user_id is None if unauthenticated
        let user_id = match value.get("GSI1PK") {
            Some(user_id_value) => Some(split_at_hash(user_id_value.as_s().unwrap()).to_string()),
            None => None,
        };
        Session {
            id: split_at_hash(value.get("PK").unwrap().as_s().unwrap()).to_string(),
            user_id,
        }
    }
}

impl From<HashMap<String, AV>> for Team {
    fn from(value: HashMap<String, AV>) -> Self {
        Team {
            id: split_at_hash(value.get("PK").unwrap().as_s().unwrap()).to_string(),
            name: split_at_hash(value.get("GSI1PK").unwrap().as_s().unwrap()).to_string(),
            active: *value.get("is_active").unwrap().as_bool().unwrap(),
        }
    }
}

impl From<HashMap<String, AV>> for Org {
    fn from(value: HashMap<String, AV>) -> Self {
        Org {
            id: split_at_hash(value.get("PK").unwrap().as_s().unwrap()).to_string(),
            name: split_at_hash(value.get("GSI1PK").unwrap().as_s().unwrap()).to_string(),
            active: *value.get("is_active").unwrap().as_bool().unwrap(),
        }
    }
}

impl From<HashMap<String, AV>> for ConnectorDetails {
    fn from(value: HashMap<String, AV>) -> Self {
        ConnectorDetails {
            id: split_at_hash(value.get("PK").unwrap().as_s().unwrap()).to_string(),
            name: split_at_hash(value.get("GSI1PK").unwrap().as_s().unwrap()).to_string(),
            connection_string: value
                .get("connection_string")
                .unwrap()
                .as_s()
                .unwrap()
                .to_string(),
            r#type: value
                .get("connector_type")
                .unwrap()
                .as_s()
                .unwrap()
                .to_string()
                .into(),
        }
    }
}

impl From<HashMap<String, AV>> for Dataset {
    fn from(value: HashMap<String, AV>) -> Self {
        Dataset {
            id: split_at_hash(value.get("PK").unwrap().as_s().unwrap()).to_string(),
            name: split_at_hash(value.get("GSI1PK").unwrap().as_s().unwrap()).to_string(),
            provider: Some(
                value
                    .get("provider")
                    .unwrap_or(&AV::S("".to_string()))
                    .as_s()
                    .unwrap()
                    .to_string(),
            ),
            connector_id: value
                .get("connector_id")
                .unwrap()
                .as_s()
                .unwrap()
                .to_string(),
            path: value.get("path").unwrap().as_s().unwrap().to_string(),
            description: value
                .get("description")
                .unwrap()
                .as_s()
                .unwrap()
                .to_string(),
            schema: serde_json::from_str(value.get("schema").unwrap().as_s().unwrap()).unwrap(),
            tags: serde_json::from_str(value.get("tags").unwrap().as_s().unwrap()).unwrap(),
            metadata: value
                .get("metadata")
                .map(|metadata| {
                    let metadata_str = metadata.as_s().unwrap();
                    if metadata_str.is_empty() {
                        None
                    } else {
                        Some(serde_json::from_str(metadata_str).unwrap())
                    }
                })
                .unwrap_or(None),
        }
    }
}
