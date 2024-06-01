use crate::core::connector::{self, Trait};
use crate::core::user;
use crate::core::PostgresConnector;
use crate::data::Database;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;

pub async fn create<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<user::Extension>,
    Json(payload): Json<connector::Create>,
) -> impl IntoResponse {
    if let Some(user) = user_ext.user {
        if user.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    } else {
        return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
    }

    match payload.r#type {
        connector::Type::Postgres => {
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

pub async fn get<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<user::Extension>,
) -> impl IntoResponse {
    if let Some(user) = user_ext.user {
        if user.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    } else {
        return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
    }

    match connector::Details::get_connector_details(state.db, true).await {
        Ok(connector_details) => (StatusCode::OK, Json(json!(connector_details))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}

pub async fn all_datasets<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<user::Extension>,
    Path(connector_id): Path<String>,
) -> impl IntoResponse {
    if let Some(user) = user_ext.user {
        if user.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    } else {
        return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
    }

    match state
        .connections
        .get(&connector_id)
        .unwrap()
        .get_available_datasets()
        .await
    {
        Ok(connector_details) => (StatusCode::OK, Json(json!(connector_details))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e.to_string())),
        )
            .into_response(),
    }
}
