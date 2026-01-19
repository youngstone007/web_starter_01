use crate::App::AppState;
use axum::response::IntoResponse;
use axum::Router;
use sea_orm::EntityTrait;

mod user;

pub fn create_api_router() -> Router<AppState> {
    Router::new()
        .nest("/api", Router::new().nest("/user", user::create_router()))
        .fallback(|| -> anyhow::Result<()> {
            anyhow::bail!("Not found")
        })
}


use axum::{
    http::StatusCode,
    response::{ Response},
};

struct CustomError(String);

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0).into_response()
    }
}

async fn handler() -> Result<&'static str, CustomError> {
    Err(CustomError("Something went wrong".to_string()))
}