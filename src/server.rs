use crate::config::ServerConfig;
use crate::App::AppState;
use anyhow::Context;
use axum::{debug_handler, routing, Router};
use sea_orm::Iden;
use tokio::net::TcpListener;
use tracing::instrument::WithSubscriber;
use tracing_subscriber::Registry;

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Server { config }
    }

    pub fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        axum::Router::new().merge(router).with_state(state)
    }

    pub async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let router = self.build_router(state, router);

        let port = self.config.port();

        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

        tracing::info!("server started on port {}", port);

        axum::serve(listener, router).await?;

        Ok(())
    }
}
