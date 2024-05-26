use crate::core::{
    ConnectorDetails, ConnectorTrait, ConnectorType, CreateConnector, PostgresConnector,
    UserExtension,
};
use crate::data::Database;
use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde_json::json;

pub async fn create_connector<D: Database>(
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
                CreateConnector {
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

pub async fn get_connectors<D: Database>(
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
