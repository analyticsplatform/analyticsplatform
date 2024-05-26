use crate::core::{Session, User};
use crate::data::Database;
use crate::AppState;

use argon2::{password_hash::PasswordHash, Argon2, PasswordVerifier};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn login<D: Database>(
    State(state): State<AppState<D>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    match User::from_email(state.db.clone(), &payload.email).await {
        Ok(user) => {
            if let Ok(parsed_hash) = PasswordHash::new(&user.hash) {
                if Argon2::default()
                    .verify_password(&payload.password.into_bytes(), &parsed_hash)
                    .is_ok()
                {
                    if let Ok(session) = Session::create(state.db, Some(&user)).await {
                        return (
                            StatusCode::OK,
                            Json(LoginResponse { token: session.id }).into_response(),
                        );
                    }
                }
            }
            (StatusCode::UNAUTHORIZED, "auth failed".into_response())
        }
        Err(_) => {
            info!("USER: search failed");
            (StatusCode::UNAUTHORIZED, "ERROR: AUTH".into_response())
        }
    }
}

pub async fn anonymous_login<D: Database>(State(state): State<AppState<D>>) -> impl IntoResponse {
    if let Ok(session) = Session::create(state.db, None).await {
        (
            StatusCode::OK,
            Json(LoginResponse { token: session.id }).into_response(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to create anonymous session".into_response(),
        )
    }
}

pub async fn logout<D: Database>(
    State(state): State<AppState<D>>,
    Extension(session): Extension<Session>,
) -> impl IntoResponse {
    let _ = Session::delete(state.db, &session.id).await;
    "logout successful".into_response()
}
