use crate::core::{
    user::{self, User},
    Session,
};
use crate::data::Database;
use crate::AppState;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tracing::info;

pub async fn auth<D: Database>(
    State(state): State<AppState<D>>,
    mut request: Request,
    next: Next,
) -> Response {
    // Paths that can be accessed by anonymous users
    let anonymous_paths: Vec<&str> = vec!["/datasets", "/profile"];
    let path: &str = request.uri().path();

    let allow_anonymous = anonymous_paths
        .iter()
        .any(|prefix| path.starts_with(prefix));
    let allow_anonymous = allow_anonymous && request.method().as_str() == "GET";

    let auth_header_value = request.headers().get("Authorization");

    let token = if let Some(value) = auth_header_value {
        let value = value.to_str().unwrap();
        if value.starts_with("Bearer ") {
            Some(value.trim_start_matches("Bearer ").to_string())
        } else {
            return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED").into_response();
        }
    } else {
        return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED").into_response();
    };

    let token = token.unwrap();
    let Ok(session) = Session::from_id(state.db.clone(), &token).await else {
        return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED").into_response();
    };

    request.extensions_mut().insert(session.clone());
    if let Some(user_id) = session.user_id {
        info!("User ID found in session: {}", user_id);
        let user_response = User::from_id(state.db, &user_id).await;
        if let Ok(user) = user_response {
            request
                .extensions_mut()
                .insert(user::Extension { user: Some(user) });
        } else {
            info!("User not found");
            return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED").into_response();
        }
    } else {
        info!("Exisiting Anonymous Session");
        if !allow_anonymous {
            return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED").into_response();
        }
        request
            .extensions_mut()
            .insert(user::Extension { user: None });
        let response = next.run(request).await;
        return (StatusCode::OK, response).into_response();
    }
    let response = next.run(request).await;
    response.into_response()
}
