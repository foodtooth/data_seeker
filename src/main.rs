mod settings;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use lazy_static::lazy_static;
use settings::Settings;
use std::sync::RwLock;

lazy_static! {
    static ref CONFIGS: RwLock<Settings> =
        RwLock::new(Settings::new().expect("configs should be ready when starting app"));
}

#[tokio::main]
async fn main() {
    // build app with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/transactions", get(get_transactions));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // 2. database connection
    let settings = CONFIGS.read().unwrap();
    println!("{:?}", settings.server.data_seeker.host);
    todo!();
}

async fn root() -> &'static str {
    "Hello, World!"
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
