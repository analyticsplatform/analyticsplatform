mod core;
mod data;
use argon2::{password_hash::PasswordHash, Argon2, PasswordVerifier};
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Extension, Json, Router,
};
use core::{CreateConnector, CreateDataset, Dataset, UserExtension};
use data::Database;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

use crate::core::{
    auth, Connector, ConnectorDetails, ConnectorTrait, ConnectorType, CreateOrg, CreateTeam,
    CreateUser, Org, PostgresConnector, Profile, Session, Team, User,
};
use crate::data::Dynamodb;

// TODO: add connections property. It will store list of
// connections/data sources which are updated during runtime
#[derive(Debug, Clone)]
struct AppState<D: Database> {
    db: D,
    connections: Arc<HashMap<String, Connector>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let _connections: Arc<HashMap<std::string::String, Connector>> = Arc::new(HashMap::new());

    let database = Dynamodb::new(true, &"analyticsplatform").await.unwrap();
    let connections = Arc::new(
        Connector::create_connectors(database.clone())
            .await
            .unwrap(),
    );
    let state = AppState {
        db: database,
        connections,
    };

    let app = Router::new()
        .route("/logout", post(logout))
        .route("/profile", get(profile))
        .route("/users", post(create_user))
        .route("/orgs", post(create_org))
        .route("/orgs/:org_id", get(get_org))
        .route("/orgs/:org_id", delete(delete_org))
        .route("/teams", post(create_team))
        .route("/teams/:team_id", get(get_team))
        .route("/connectors", post(create_connector))
        .route("/connectors", get(get_connectors))
        .route("/datasets", get(get_datasets))
        .route("/dataset", post(create_dataset))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(state.clone(), auth)))
        .route("/login", post(login))
        .route("/anonymouslogin", post(anonymous_login))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

async fn login<D: Database>(
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

async fn anonymous_login<D: Database>(State(state): State<AppState<D>>) -> impl IntoResponse {
    let session = Session::create(state.db, None).await.unwrap();
    let response = LoginResponse { token: session.id };
    (StatusCode::OK, Json(response).into_response())
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
async fn profile(Extension(user_ext): Extension<UserExtension>) -> impl IntoResponse {
    if let Some(u) = user_ext.user {
        Json(Profile::from(u)).into_response()
    } else {
        Json("None").into_response()
    }
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

async fn create_org<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateOrg>,
) -> impl IntoResponse {
    match user.r#type.as_str() {
        "superadmin" => {
            println!("creating org.");
            match Org::create(state.db, &payload).await {
                Ok(_) => {
                    return (StatusCode::OK, "org created".into());
                }
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "org creation failed".into(),
                    );
                }
            }
        }
        _ => (StatusCode::UNAUTHORIZED, "org creation not permitted"),
    }
}

async fn get_org<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Path(org_id): Path<String>,
) -> impl IntoResponse {
    if user.r#type != "superadmin" {
        return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
    }
    // Get org from DB
    let org = Org::from_id(state.db, &org_id).await.unwrap();
    (StatusCode::OK, Json(org)).into_response()
}

async fn delete_org<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Path(org_id): Path<String>,
) -> impl IntoResponse {
    if user.r#type != "superadmin" {
        return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
    }
    let _ = Org::delete(state.db, &org_id).await;
    (StatusCode::OK, "ORG DELETED").into_response()
}

async fn create_connector<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<UserExtension>,
    Json(payload): Json<CreateConnector>,
) -> impl IntoResponse {
    println!("payload: {:?}", payload);
    let user = if let Some(u) = user_ext.user {
        if u.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    };

    println!("user: {:?}", user);

    let _ = match payload.r#type {
        ConnectorType::Postgres => {
            println!("postgres create");
            PostgresConnector::create_record(
                state.db,
                core::CreateConnector {
                    name: payload.name,
                    r#type: payload.r#type,
                    connection_string: payload.connection_string,
                },
            )
            .await
        }
    };

    (StatusCode::OK, "CREATED").into_response()
}

async fn get_connectors<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<UserExtension>,
) -> impl IntoResponse {
    let user = if let Some(u) = user_ext.user {
        if u.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    };
    println!("user: {:?}", user);

    let connector_details = ConnectorDetails::get_connector_details(state.db, true)
        .await
        .unwrap();
    println!("conns: {:?}", connector_details);

    (StatusCode::OK, Json(json!(connector_details))).into_response()
}

// TODO: Accept anonymous users with UserExtension
async fn get_datasets<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<UserExtension>,
) -> impl IntoResponse {
    let _user = if let Some(u) = user_ext.user {
        if u.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    };

    let datasets = Dataset::get_all(state.db).await.unwrap();

    (StatusCode::OK, Json(json!(datasets))).into_response()
}

async fn create_dataset<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user_ext): Extension<UserExtension>,
    Json(payload): Json<CreateDataset>,
) -> impl IntoResponse {
    let _user = if let Some(u) = user_ext.user {
        if u.r#type != "superadmin" {
            return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
        }
    };

    Dataset::create(state, payload).await.unwrap();
    StatusCode::OK.into_response()
}

async fn create_team<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateTeam>,
) -> impl IntoResponse {
    match user.r#type.as_str() {
        "superadmin" => {
            println!("creating team.");
            match Team::create(state.db, &payload).await {
                Ok(_) => {
                    return (StatusCode::OK, "team created".into());
                }
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "team creation failed".into(),
                    );
                }
            }
        }
        _ => (StatusCode::UNAUTHORIZED, "team creation not permitted"),
    }
}

async fn get_team<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Path(team_id): Path<String>,
) -> impl IntoResponse {
    if user.r#type != "superadmin" {
        return (StatusCode::UNAUTHORIZED, Json(json!("UNAUTHORIZED"))).into_response();
    }
    // TODO: Allow team members to view own teams
    let team = Team::from_id(state.db, &team_id).await.unwrap();
    (StatusCode::OK, Json(team)).into_response()
}
