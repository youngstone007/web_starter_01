use crate::config::ServerConfig;
use anyhow::Context;
use axum::Router;
use sea_orm::Iden;
use tokio::net::TcpListener;

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Server { config }
    }

    pub fn build_router(&self) -> Router {
        todo!()
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let router = self.build_router();

        let port = self.config.port();

        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

        tracing::info!("server started on port {}", port);

        axum::serve(listener, router).await?;

        Ok(())
    }
}
