mod App;
mod api;
mod config;
mod database;
mod entity;
mod logger;
mod server;

use crate::api::create_api_router;
use axum::response::IntoResponse;
use axum::{debug_handler, routing, Router};
use sea_orm::prelude::*;
use sea_orm::sea_query::ExprTrait;
use sea_orm::QueryTrait;
use std::ops::Deref;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    App::run(create_api_router()).await
}
