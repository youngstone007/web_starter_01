use crate::api::create_api_router;

mod App;
mod api;
mod config;
mod database;
mod entity;
mod error;
mod logger;
mod response;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    App::run(create_api_router()).await
}
