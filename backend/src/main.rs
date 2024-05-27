mod core;
mod data;
mod routes;
use axum::{
    debug_handler,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
    Router,
};
use data::Database;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::core::{auth, Connector};
use crate::data::Dynamodb;

// TODO: add connections property. It will store list of
// connections/data sources which are updated during runtime
#[derive(Debug, Clone)]
pub struct AppState<D: Database> {
    db: D,
    connections: Arc<HashMap<String, Connector>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let local = env::var("LOCAL").is_ok();
    let table_name = env::var("TABLE_NAME").unwrap();
    let _connections: Arc<HashMap<std::string::String, Connector>> = Arc::new(HashMap::new());

    let database = Dynamodb::new(local, &table_name).await.unwrap();
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
        .route("/logout", post(routes::auth::logout))
        .route("/profile", get(routes::user::profile))
        .route("/users", post(routes::user::create))
        .route("/orgs", post(routes::org::create))
        .route("/orgs/:org_id", get(routes::org::get))
        .route("/orgs/:org_id", delete(routes::org::delete))
        .route("/teams", post(routes::team::create))
        .route("/teams/:team_id", get(routes::team::get))
        .route("/connectors", post(routes::connector::create))
        .route("/connectors", get(routes::connector::get))
        .route("/datasets", get(routes::dataset::get))
        .route("/dataset", post(routes::dataset::create))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(state.clone(), auth)))
        .route("/login", post(routes::auth::login))
        .route("/anonymouslogin", post(routes::auth::anonymous_login))
        .route("/health", get(health))
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn health() -> impl IntoResponse {
    (StatusCode::OK, "healthy").into_response()
}
