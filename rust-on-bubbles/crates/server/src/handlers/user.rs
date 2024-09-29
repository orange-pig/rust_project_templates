use axum::{
    http::StatusCode,
    Json,
};

use  crate::models::user::User;

#[axum_macros::debug_handler]
pub async fn get_user() -> Result<Json<User>, StatusCode> {
    Ok(Json(User {
        name: "alpha".to_string(),
        email: "a@a.com".to_string(),
    }))
}
