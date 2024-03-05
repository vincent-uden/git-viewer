use std::path::PathBuf;

use axum::{
    routing::get,
    Router,
};

mod index;

use index::index;
use serde::Deserialize;
use tower_http::services::ServeDir;

#[derive(Deserialize, Debug)]
struct Config {
    git_root: PathBuf,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let config = match envy::from_env::<Config>() {
        Ok(c) => c,
        Err(e) => {
            panic!("Error parsing .env file\n{}", e);
        }
    };

    let serve_dir = ServeDir::new("assets");

    let app = Router::new().route("/", get(index)).fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}