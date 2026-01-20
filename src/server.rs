use std::time::Duration;

use crate::App::AppState;
use crate::config::ServerConfig;
use anyhow::Context;
use axum::{
    Router, debug_handler,
    extract::{DefaultBodyLimit, Request},
    routing,
};
use bytesize::ByteSize;
use tokio::net::TcpListener;
use tower_http::{
    cors::{self, CorsLayer},
    normalize_path::NormalizePathLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Server { config }
    }

    pub fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        let timeout = TimeoutLayer::new(Duration::from_secs(120));
        let body_limit = DefaultBodyLimit::max(ByteSize::mib(10).as_u64() as usize);

        let cors = CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_methods(cors::Any)
            .allow_headers(cors::Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(3600 * 12));

        let tracing = TraceLayer::new_for_http()
            .make_span_with(|request: &Request| {
                let method = request.method();
                let path = request.uri().path();
                let id = xid::new();

                tracing::info_span!("Api Request", id = %id, method = %method, path = %path)
            })
            .on_request(())
            .on_failure(())
            .on_response(());

        let normalize_path = NormalizePathLayer::trim_trailing_slash();

        Router::new()
            .merge(router)
            .layer(timeout)
            .layer(body_limit)
            .layer(tracing)
            .layer(cors)
            .layer(normalize_path)
            .with_state(state)
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
