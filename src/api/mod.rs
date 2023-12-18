use axum::{http::StatusCode, response::IntoResponse, Json};
use diesel_async::{
    pooled_connection::deadpool::{Pool, PoolError},
    AsyncPgConnection,
};
use uuid::Uuid;

pub mod user;
pub mod workspace;

#[derive(Clone)]
pub struct ApiState {
    pub db_conn_pool: Pool<AsyncPgConnection>,
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    NotFound(Uuid),
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_code, err_msg) = match self {
            Self::BadRequest(err_msg) => {
                (StatusCode::BAD_REQUEST, String::from("BadRequest"), err_msg)
            }
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                String::from("NotFound"),
                format!("Model with id {} could not be found", id),
            ),
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("InternalServerError"),
                String::from("Internal Server Error"),
            ),
        };

        (
            status,
            Json(serde_json::json!(
                {"error_code": err_code, "error_message": err_msg}
            )),
        )
            .into_response()
    }
}

impl From<PoolError> for ApiError {
    fn from(_: PoolError) -> Self {
        Self::InternalServerError
    }
}
