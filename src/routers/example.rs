use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};

use crate::{entities::Example, errors::ResponseError, AppState};

/// This is an example router to use the Database entity.
pub fn build() -> Router<AppState> {
    Router::new().route("/example", get(example_handler))
}

/// Returns the first Example entity.
#[axum::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn example_handler(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, ResponseError> {
    let res = sqlx::query_as!(Example, "select * from example limit 1")
        .fetch_one(&app_state.database)
        .await?;

    Ok(Json(res))
}
