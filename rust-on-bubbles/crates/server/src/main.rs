use axum::{routing::get, Router};
use db::hello_db;
use handlers::user::get_user;
use tokio::net::TcpListener;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    
    // ## init DB pool
    let _pool = hello_db().await.expect("Can't connect to database");

    let user_router = Router::new().route("/", get(get_user));

    // ## build application
    let app = Router::new().nest("/user", user_router);

    // ## run app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
