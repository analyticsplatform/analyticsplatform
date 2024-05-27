use crate::core::{
    org::{Create, Org},
    User,
};
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
    Extension(user): Extension<User>,
    Json(payload): Json<Create>,
) -> impl IntoResponse {
    if user.r#type != "superadmin" {
        return (StatusCode::UNAUTHORIZED, "org creation not permitted").into_response();
    }

    println!("creating org.");
    match Org::create(state.db, &payload).await {
        Ok(()) => (StatusCode::OK, "org created").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "org creation failed").into_response(),
    }
}

pub async fn get<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Path(org_id): Path<String>,
) -> impl IntoResponse {
    if user.r#type != "superadmin" {
        return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
    }

    match Org::from_id(state.db, &org_id).await {
        Ok(org) => (StatusCode::OK, Json(org)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Json(json!("ORG NOT FOUND"))).into_response(),
    }
}

pub async fn delete<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Path(org_id): Path<String>,
) -> impl IntoResponse {
    if user.r#type != "superadmin" {
        return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
    }

    match Org::delete(state.db, &org_id).await {
        Ok(()) => (StatusCode::OK, "ORG DELETED").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "ORG DELETION FAILED").into_response(),
    }
}
