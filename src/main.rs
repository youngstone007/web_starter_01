mod config;
mod database;
mod entity;
mod logger;
mod server;

use std::error;
use crate::entity::sys_user;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{debug_handler, routing, Router};
use entity::prelude::*;
use sea_orm::prelude::*;
use sea_orm::sea_query::ExprTrait;
use sea_orm::{Condition, DatabaseBackend, QueryTrait};
use std::ops::Deref;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    logger::init();


    // 数据库初始化
    let db = database::init_database()
        .await
        .expect("database init wrong!!!");

    let cfg = config::config();

    let port = cfg.server().port();

    let router = Router::new()
        .route("/", routing::get(index))
        .route("/user-all", routing::get(query_all_user))
        .with_state(db);

    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    tracing::info!("server started on '127.0.0.1:{port}' ====>>>");

    axum::serve(listener, router).await.unwrap();
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello, world!"
}

#[debug_handler]
async fn query_all_user(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    let users = SysUser::find()
        .filter(
            Condition::any()
                .add(sys_user::Column::NickName.eq("赵萌".to_string()))
                .add(sys_user::Column::NickName.eq("李超".to_string())),
        )
        .all(&db)
        .await
        .unwrap();
    axum::Json(users)
}
