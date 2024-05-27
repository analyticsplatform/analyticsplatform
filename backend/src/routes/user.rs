use crate::core::{
    user::{self, User},
    Profile,
};
use crate::data::Database;
use crate::AppState;
use axum::{
    debug_handler, extract::State, http::StatusCode, response::IntoResponse, Extension, Json,
};

#[debug_handler]
pub async fn profile(Extension(user_ext): Extension<user::Extension>) -> impl IntoResponse {
    Json(user_ext.user.map(Profile::from)).into_response()
}

pub async fn create<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Json(payload): Json<user::Create>,
) -> impl IntoResponse {
    match user.r#type.as_str() {
        "superadmin" => {
            println!("creating user.");
            if let Ok(()) = User::create(state.db, &payload).await {
                println!("user created successfully");
                (StatusCode::OK, "user created")
            } else {
                println!("user creation failed");
                (StatusCode::INTERNAL_SERVER_ERROR, "Error")
            }
        }
        _ => (StatusCode::UNAUTHORIZED, "user creation not permitted"),
    }
}
