mod core;
mod data;
use argon2::{password_hash::PasswordHash, Argon2, PasswordVerifier};
use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use data::Database;
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_cookies::{Cookie, CookieManagerLayer};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::core::{auth, CreateUser, Profile, Session, User};
use crate::data::Dynamodb;

#[derive(Debug, Clone)]
struct AppState<D: Database> {
    db: D,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database = Dynamodb::new(true, &"tower").await.unwrap();
    let state = AppState { db: database };

    let app = Router::new()
        .route("/logout", post(logout))
        .route("/profile", get(profile))
        .route("/users", post(create_user))
        .layer(
            ServiceBuilder::new()
                .layer(CookieManagerLayer::new())
                .layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
        .route("/login", post(login))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

async fn login<D: Database>(
    State(state): State<AppState<D>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    println!("{payload:?}");
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

            let session = Session::create(state.db, &user).await.unwrap();
            let cookie = Cookie::build(("sid", session.id))
                .path("/")
                .http_only(true)
                .build();
            let mut response: Response = Response::new("auth successful".into());
            response
                .headers_mut()
                .insert("Set-Cookie", cookie.to_string().parse().unwrap());

            (StatusCode::OK, response)
        }
        Err(_) => {
            println!("user search failed");
            let response: Response = Response::new("auth failed".into());
            (StatusCode::UNAUTHORIZED, response)
        }
    }
}

async fn logout<D: Database>(
    State(state): State<AppState<D>>,
    Extension(session): Extension<Session>,
) -> impl IntoResponse {
    // TODO: Read sid value from cookie
    let _ = Session::delete(state.db, &session.id).await;
    "logout successful".into_response()
}

#[debug_handler]
async fn profile(Extension(user): Extension<User>) -> impl IntoResponse {
    println!("{user:?}");
    Json(Profile::from(user))
}

async fn create_user<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    match user.r#type.as_str() {
        "superadmin" => {
            println!("creating user.");
            match User::create(state.db, &payload).await {
                Ok(_) => {
                    println!("user created successfully");
                    return (StatusCode::OK, "user created".into());
                }
                Err(_) => {
                    ("user creation failed");
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Error".into());
                }
            }
        }
        _ => (StatusCode::UNAUTHORIZED, "user creation not permitted"),
    }
}
