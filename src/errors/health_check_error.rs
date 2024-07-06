use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum HealthCheckError {
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),
}

impl IntoResponse for HealthCheckError {
    fn into_response(self) -> Response {
        error!("health check error: {}", self);

        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(self.to_string()))
            .expect("Response Builder with known setup should not fail")
            .into_response()
    }
}
