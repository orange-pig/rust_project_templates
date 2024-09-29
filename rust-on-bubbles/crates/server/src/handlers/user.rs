use crate::{app_state::AppState, models::user::User};
use axum::{extract::State, http::StatusCode, Json};
use db::user::{query_user, UserEntity};

#[axum_macros::debug_handler]
pub async fn get_user(State(state): State<AppState>) -> Result<Json<User>, StatusCode> {
    let acc: Option<UserEntity> = query_user(state.db.pool).await;

    if let Some(acc) = acc {
        return Ok(Json(User {
            name: acc.name,
            email: acc.email,
        }));
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
