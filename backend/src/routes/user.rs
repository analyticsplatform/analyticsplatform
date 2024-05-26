use crate::core::{CreateUser, Profile, User, UserExtension};
use crate::data::Database;
use crate::AppState;
use axum::{
    debug_handler, extract::State, http::StatusCode, response::IntoResponse, Extension, Json,
};

#[debug_handler]
pub async fn profile(Extension(user_ext): Extension<UserExtension>) -> impl IntoResponse {
    Json(user_ext.user.map(Profile::from)).into_response()
}

pub async fn create_user<D: Database>(
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
                    (StatusCode::OK, "user created")
                }
                Err(_) => {
                    println!("user creation failed");
                    (StatusCode::INTERNAL_SERVER_ERROR, "Error")
                }
            }
        }
        _ => (StatusCode::UNAUTHORIZED, "user creation not permitted"),
    }
}
