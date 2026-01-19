use crate::config::config;
use crate::server::Server;
use crate::{database, logger};
use axum::Router;
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        AppState { db }
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();

    // 数据库初始化
    let db = database::init_database().await?;
    let state = AppState::new(db);

    let server = Server::new(config().server());
    let port = config().server().port();
    tracing::info!("server started on '127.0.0.1:{port}' ====>>>");

    server.start(state, router).await
}
