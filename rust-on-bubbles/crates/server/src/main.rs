use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // ## 4. build application
    let app = Router::new()
        .route("/", get(Html("<h1>Hello World!</h1>")))
        .route("/hello", get(|| async { "Hello Rust on Bubble!" }));

    // ## 5. run app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
