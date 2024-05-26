use crate::core::{CreateDataset, Dataset, UserExtension};
use crate::data::Database;
use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;

// TODO: Accept anonymous users with UserExtension

pub async fn get_datasets<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<UserExtension>,
) -> impl IntoResponse {
    if let Some(user) = user_ext.user {
        if user.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    }

    match Dataset::get_all(state.db).await {
        Ok(datasets) => (StatusCode::OK, Json(json!(datasets))).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_dataset<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<UserExtension>,
    Json(payload): Json<CreateDataset>,
) -> impl IntoResponse {
    if let Some(user) = user_ext.user {
        if user.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    }

    match Dataset::create(state, payload).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
