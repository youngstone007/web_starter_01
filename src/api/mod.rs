use crate::error::{ApiError, ApiResult};
use crate::App::AppState;
use axum::Router;

mod user;

pub fn create_api_router() -> Router<AppState> {
    Router::new()
        .nest(
            "/api",
            Router::new().nest("/user", user::create_router()).fallback(
                async || -> ApiResult<()> {
                    tracing::error!("method not found");
                    Err(ApiError::NotFound)
                },
            ),
        )
        .method_not_allowed_fallback(async || -> ApiResult<()> {
            tracing::warn!("Method not allowed");
            Err(ApiError::MethodNotAllowed)
        })
}
