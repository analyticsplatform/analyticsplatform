use crate::core::{CreateTeam, Team, User};
use crate::data::Database;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;

pub async fn create_team<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateTeam>,
) -> impl IntoResponse {
    if user.r#type != "superadmin" {
        return (StatusCode::UNAUTHORIZED, "team creation not permitted").into_response();
    }

    println!("creating team.");
    match Team::create(state.db, &payload).await {
        Ok(_) => (StatusCode::OK, "team created").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "team creation failed").into_response(),
    }
}

pub async fn get_team<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Path(team_id): Path<String>,
) -> impl IntoResponse {
    if user.r#type != "superadmin" {
        return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
    }

    // TODO: Allow team members to view own teams
    match Team::from_id(state.db, &team_id).await {
        Ok(team) => (StatusCode::OK, Json(team)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Json(json!("Team not found"))).into_response(),
    }
}
