mod settings;

use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, FromRequest, Json, Path, Query},
    http::{Method, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use lazy_static::lazy_static;
use settings::Settings;
use std::sync::RwLock;
use validator::Validate;

lazy_static! {
    pub static ref CONFIGS: RwLock<Settings> =
        RwLock::new(Settings::new().expect("configs should be ready when starting app"));
}

pub fn app() -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    // build app with routes
    Router::new()
        .route("/health_check", get(health_check))
        .route("/transactions/:transaction_id", get(get_transactions))
        .route("/transaction", post(create_transaction))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors),
        )
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn get_transactions(
    Path(transaction_id): Path<String>,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse {
    dbg!(transaction_id);
    // fake transaction
    let transaction = Transaction {
        transaction_id: "01234".to_string(),
        status: "ok".to_string(),
        r#type: "transfer".to_string(),
    };

    let Query(pagination) = pagination.unwrap_or_default();

    (StatusCode::OK, Json(transaction))
}

#[derive(Deserialize)]
struct GetTransaction {
    transaction_id: String,
}

#[derive(Deserialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

async fn create_transaction(
    ValidatedJson(payload): ValidatedJson<CreateTransaction>,
) -> impl IntoResponse {
    // dbg!(payload);
    // fake transaction
    let transaction = Transaction {
        transaction_id: "01234".to_owned(),
        status: payload.status,
        r#type: "payload.r#type".to_owned(),
    };

    (StatusCode::CREATED, Json(transaction))
}

#[derive(Deserialize, Validate)]
struct CreateTransaction {
    #[validate(length(min = 1, message = "Can not be empty"))]
    status: String,
    r#type: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    B: Send + 'static,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumJsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}

#[derive(Serialize)]
struct Transaction {
    transaction_id: String,
    status: String,
    r#type: String,
}
