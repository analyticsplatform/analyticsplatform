use crate::core::{Session, User};
use crate::data::Database;
use crate::AppState;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use tower_cookies::{Cookie, Cookies};

pub async fn auth<D: Database>(
    State(state): State<AppState<D>>,
    cookies: Cookies,
    mut request: Request,
    next: Next,
) -> impl IntoResponse {
    // Paths that can be accessed by anonymous users
    let anonymous_paths: Vec<&str> = vec!["/datasets"];
    let path: &str = request.uri().path();

    let allow_anonymous = anonymous_paths
        .iter()
        .any(|prefix| path.starts_with(prefix));
    let allow_anonymous = allow_anonymous && request.method().as_str() == "GET";

    if cookies.get("sid").is_none() && allow_anonymous {
        println!("New Anonymous Access: Creating anonymous session");
        let session = Session::create(state.db, None).await.unwrap();
        let cookie = Cookie::build(("sid", session.id))
            .path("/")
            .http_only(true)
            .build();
        let mut response = next.run(request).await;
        response
            .headers_mut()
            .insert("Set-Cookie", cookie.to_string().parse().unwrap());

        return (StatusCode::OK, response);
    }

    if cookies.get("sid").is_none() && !allow_anonymous {
        return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED".into_response());
    }

    let sid_cookie = cookies.get("sid").unwrap();
    let session = match Session::from_id(state.db.clone(), sid_cookie.value()).await {
        Ok(s) => s,
        Err(_) => return (StatusCode::UNAUTHORIZED, "Session Error".into_response()),
    };

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
                    return (StatusCode::UNAUTHORIZED, "User Error".into_response());
                }
            }
        }
        None => {
            println!("Exisiting Anonymous Session");
            let response = next.run(request).await;
            return (StatusCode::OK, response);
        }
    }
    let response = next.run(request).await;
    // do something with `response`...
    (StatusCode::OK, response)
}
