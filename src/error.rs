use actix_web::{HttpResponse, http::StatusCode};
use deadpool_postgres::tokio_postgres;
use serde_json::{Value, json};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum Error {
    ConnectPostgres(#[from] deadpool_postgres::CreatePoolError),
    DeadpoolPool(#[from] deadpool_postgres::PoolError),
    Io(#[from] std::io::Error),
    Postgres(#[from] tokio_postgres::Error),

    DocumentAlreadyExists(String),
    NeedDataPath(Value),
}

impl actix_web::error::ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::DocumentAlreadyExists(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::NeedDataPath(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({"error": format!("{:?}", self)}))
    }
}
