use axum::{
    routing::get,
    Router,
};

mod index;

use index::index;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let serve_dir = ServeDir::new("assets");

    let app = Router::new().route("/", get(index)).fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}