
use crate::entity::prelude::*;
use crate::App::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{debug_handler, routing, Router};
use sea_orm::prelude::*;
use sea_orm::{Condition, EntityTrait};
use crate::entity::sys_user;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/user-all", routing::get(query_all_user))
}

#[debug_handler]
async fn query_all_user(state: State<AppState>) -> impl IntoResponse {
    let users = SysUser::find()
        .filter(
            Condition::any()
                .add(sys_user::Column::NickName.eq("赵萌".to_string()))
                .add(sys_user::Column::NickName.eq("李超".to_string())),
        )
        .all(&state.0.db)
        .await
        .unwrap();
    axum::Json(users)
}
