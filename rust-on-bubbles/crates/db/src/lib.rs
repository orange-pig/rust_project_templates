use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Error, PgPool};
use std::{env, time::Duration};

pub mod user;

#[derive(Clone)]
pub struct DbState {
    pub pool: PgPool,
}

pub async fn connc_db() -> Result<PgPool, Error> {
    dotenv().expect(".env not found!");
    let db_connection_str: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
}
