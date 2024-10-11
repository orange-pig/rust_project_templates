use app_state::AppState;
use axum::{routing::get, Router};
use db::{connc_db, DbState};
use handlers::user::get_user;
use tokio::net::TcpListener;

mod app_state;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // ## init DB pool
    let pool = connc_db().await.expect("Can't connect to database");

    // ## init app state
    let state: AppState = AppState {
        db: DbState { pool: pool },
    };

    let user_router = Router::new().route("/", get(get_user));

    // ## build application
    let app = Router::new().nest("/user", user_router).with_state(state);

    // ## run app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
