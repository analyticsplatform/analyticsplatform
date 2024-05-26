use crate::core::{CreateOrg, Org, User};
use crate::data::Database;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;

pub async fn create_org<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateOrg>,
) -> impl IntoResponse {
    match user.r#type.as_str() {
        "superadmin" => {
            println!("creating org.");
            match Org::create(state.db, &payload).await {
                Ok(_) => {
                    return (StatusCode::OK, "org created".into());
                }
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "org creation failed".into(),
                    );
                }
            }
        }
        _ => (StatusCode::UNAUTHORIZED, "org creation not permitted"),
    }
}

pub async fn get_org<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Path(org_id): Path<String>,
) -> impl IntoResponse {
    if user.r#type != "superadmin" {
        return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
    }
    // Get org from DB
    let org = Org::from_id(state.db, &org_id).await.unwrap();
    (StatusCode::OK, Json(org)).into_response()
}

pub async fn delete_org<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Path(org_id): Path<String>,
) -> impl IntoResponse {
    if user.r#type != "superadmin" {
        return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
    }
    let _ = Org::delete(state.db, &org_id).await;
    (StatusCode::OK, "ORG DELETED").into_response()
}
