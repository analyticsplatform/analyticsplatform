mod core;
mod data;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use cookie::Cookie;
use serde::{Deserialize, Serialize};

use crate::core::{Session, User};
use crate::data::Dynamodb;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database = Dynamodb::new(true, &"tower").await.unwrap();
    let app = Router::new()
        .route("/login", post(login))
        .with_state(database);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

async fn login(State(db): State<Dynamodb>, Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    println!("{payload:?}");
    let user_response = User::from_email(db.clone(), &payload.email).await;
    println!("{user_response:?}");

    match user_response {
        Ok(user) => {
            let session = Session::create(db, &user).await.unwrap();
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
