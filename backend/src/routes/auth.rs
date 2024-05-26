use crate::core::{Session, User};
use crate::data::Database;
use crate::AppState;
use argon2::{password_hash::PasswordHash, Argon2, PasswordVerifier};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
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
    let user_response = User::from_email(state.db.clone(), &payload.email).await;

    match user_response {
        Ok(user) => {
            let parsed_hash = PasswordHash::new(&user.hash).unwrap();
            let pass_match =
                Argon2::default().verify_password(&payload.password.into_bytes(), &parsed_hash);
            if pass_match.is_err() {
                let response: Response = Response::new("auth failed".into());
                return (StatusCode::UNAUTHORIZED, response);
            }

            let session = Session::create(state.db, Some(&user)).await.unwrap();
            let response = LoginResponse { token: session.id };

            (StatusCode::OK, Json(response).into_response())
        }
        Err(_) => {
            info!("USER: search failed");
            let response: Response = Response::new("ERROR: AUTH".into());
            (StatusCode::UNAUTHORIZED, response)
        }
    }
}

pub async fn anonymous_login<D: Database>(State(state): State<AppState<D>>) -> impl IntoResponse {
    let session = Session::create(state.db, None).await.unwrap();
    let response = LoginResponse { token: session.id };
    (StatusCode::OK, Json(response).into_response())
}

pub async fn logout<D: Database>(
    State(state): State<AppState<D>>,
    Extension(session): Extension<Session>,
) -> impl IntoResponse {
    // TODO: Read sid value from cookie
    let _ = Session::delete(state.db, &session.id).await;
    "logout successful".into_response()
}
