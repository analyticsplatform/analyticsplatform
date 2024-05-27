use crate::core::{
    dataset::{Create, Dataset},
    user,
};
use crate::data::Database;
use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;

// TODO: Accept anonymous users with UserExtension

pub async fn get<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<user::Extension>,
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

pub async fn create<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<user::Extension>,
    Json(payload): Json<Create>,
) -> impl IntoResponse {
    if let Some(user) = user_ext.user {
        if user.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    }

    match Dataset::create(state, payload).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
