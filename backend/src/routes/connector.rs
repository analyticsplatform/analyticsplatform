use crate::core::{
    ConnectorDetails, ConnectorTrait, ConnectorType, CreateConnector, PostgresConnector,
    UserExtension,
};
use crate::data::Database;
use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;

pub async fn create_connector<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<UserExtension>,
    Json(payload): Json<CreateConnector>,
) -> impl IntoResponse {
    if let Some(user) = user_ext.user {
        if user.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    } else {
        return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
    }

    match payload.r#type {
        ConnectorType::Postgres => {
            if let Err(e) = PostgresConnector::create_record(state.db, payload).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!(e.to_string())),
                )
                    .into_response();
            }
        }
    }

    (StatusCode::OK, Json(json!("CREATED"))).into_response()
}

pub async fn get_connectors<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<UserExtension>,
) -> impl IntoResponse {
    if let Some(user) = user_ext.user {
        if user.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    } else {
        return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
    }

    match ConnectorDetails::get_connector_details(state.db, true).await {
        Ok(connector_details) => (StatusCode::OK, Json(json!(connector_details))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}
