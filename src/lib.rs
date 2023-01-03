mod settings;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;

use lazy_static::lazy_static;
use settings::Settings;
use std::sync::RwLock;

lazy_static! {
    static ref CONFIGS: RwLock<Settings> =
        RwLock::new(Settings::new().expect("configs should be ready when starting app"));
}

pub fn app() -> Router {
    // build app with routes
    Router::new()
        .route("/health_check", get(health_check))
        .route("/transactions", get(get_transactions))
        .layer(TraceLayer::new_for_http())

    // 2. database connection
    // let settings = CONFIGS.read().unwrap();
    // println!("{:?}", settings.server.data_seeker.host);
    // todo!();
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn get_transactions(Json(payload): Json<GetTransaction>) -> impl IntoResponse {
    // fake transaction
    let transaction = Transaction {
        transaction_id: "01234".to_string(),
        status: "ok".to_string(),
        r#type: "transfer".to_string(),
    };

    (StatusCode::OK, Json(transaction))
}

#[derive(Deserialize)]
struct GetTransaction {
    transaction_id: String,
}

#[derive(Serialize)]
struct Transaction {
    transaction_id: String,
    status: String,
    r#type: String,
}
