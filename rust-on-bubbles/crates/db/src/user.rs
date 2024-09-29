use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub async fn query_user(pool: PgPool) -> Option<UserEntity> {
    sqlx::query_as::<_, UserEntity>("SELECT * FROM users")
        .fetch_optional(&pool)
        .await
        .expect("Query error!")
}
