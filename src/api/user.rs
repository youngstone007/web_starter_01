use crate::App::AppState;
use crate::entity::prelude::*;
use crate::entity::sys_user;
use crate::error::ApiResult;
use crate::response::ApiResponse;
use axum::extract::State;
use axum::{Router, debug_handler, routing};
use sea_orm::prelude::*;
use sea_orm::{Condition, EntityTrait};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/user-all", routing::get(query_all_user))
}

#[debug_handler]
async fn query_all_user(state: State<AppState>) -> ApiResult<ApiResponse<Vec<sys_user::Model>>> {
    let users = SysUser::find()
        .filter(
            Condition::any()
                .add(sys_user::Column::NickName.eq("赵萌".to_string()))
                .add(sys_user::Column::NickName.eq("李超".to_string())),
        )
        .all(&state.0.db)
        .await?;
    Ok(ApiResponse::ok("ok", Some(users)))
}
