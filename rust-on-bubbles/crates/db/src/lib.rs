use sqlx::{postgres::PgPoolOptions, Error, PgPool};
use std::time::Duration;

#[derive(Clone)]
pub struct DbState {
    pub pool: PgPool,
}

pub async fn hello_db() -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect("postgres://your_db_username:your_db_password@localhost/your_db_name")
        .await
}
