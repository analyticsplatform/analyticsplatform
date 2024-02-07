use crate::core::{Session, User};
use crate::data::Database;
use crate::AppState;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use tower_cookies::Cookies;

pub async fn auth<D: Database>(
    State(state): State<AppState<D>>,
    cookies: Cookies,
    mut request: Request,
    next: Next,
) -> impl IntoResponse {
    match cookies.get("sid") {
        Some(sid_cookie) => {
            let session_response = Session::from_id(state.db.clone(), sid_cookie.value()).await;
            if session_response.is_err() {
                return (
                    StatusCode::UNAUTHORIZED,
                    "Session not found".into_response(),
                );
            }
            let session = session_response.unwrap();

            println!("{session:?}");
            request.extensions_mut().insert(session.clone());
            match session.user_id {
                Some(user_id) => {
                    println!("User ID found in session");
                    let user_response = User::from_id(state.db, &user_id).await;

                    match user_response {
                        Ok(user) => {
                            request.extensions_mut().insert(user);
                        }
                        Err(_) => {
                            println!("User not found");
                            return (StatusCode::UNAUTHORIZED, "User not found".into_response());
                        }
                    }
                }
                None => {
                    println!("Anonymous User")
                }
            }
        }
        None => {
            println!("sid cookie not found");
        }
    }
    request.extensions_mut().insert("user_id");
    let response = next.run(request).await;
    // do something with `response`...
    (StatusCode::OK, response)
}
