use std::path::PathBuf;

mod http;
mod index;

use http::serve;
use index::index;
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Deserialize, Debug)]
struct Config {
    git_root: PathBuf,
    database_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = match envy::from_env::<Config>() {
        Ok(c) => c,
        Err(e) => {
            panic!("Error parsing .env file\n{}", e);
        }
    };

    let db = SqlitePoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!().run(&db).await?;

    serve(config, db).await
}
