mod logger;
mod config;
mod database;

use axum::{debug_handler, routing, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    logger::init();

    // 数据库初始化
    let db = database::init_database().await.expect("database init wrong!!!");

    let cfg = config::config();

    let port = cfg.server().port();

    let router = Router::new().route("/", routing::get(index));

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await.unwrap();

    tracing::info!("server started on '127.0.0.1:{port}' ====>>>");

    axum::serve(listener, router).await.unwrap();
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello, world!"
}
