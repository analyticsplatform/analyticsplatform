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
    let _user = if let Some(u) = user_ext.user {
        if u.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    };

    let datasets = Dataset::get_all(state.db).await.unwrap();

    (StatusCode::OK, Json(json!(datasets))).into_response()
}

pub async fn create_dataset<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<UserExtension>,
    Json(payload): Json<CreateDataset>,
) -> impl IntoResponse {
    let _user = if let Some(u) = user_ext.user {
        if u.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    };

    Dataset::create(state, payload).await.unwrap();
    StatusCode::OK.into_response()
}
