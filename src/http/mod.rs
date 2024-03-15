use std::{
    slice::SliceIndex,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use sqlx::{Pool, Sqlite};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{index, repo::repo, Config};

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub db: Pool<Sqlite>,
}

pub struct AppError(anyhow::Error);
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

async fn telemetry(
    Extension(ctx): Extension<ApiContext>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    println!("{:?}", req.uri());
    let path = req.uri().to_string();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_millis() as i64;
    sqlx::query!(
        r#"insert into visits (path, created_at) values ($1, $2);"#,
        path,
        timestamp
    )
    .execute(&ctx.db)
    .await?;

    Ok(next.run(req).await)
}

pub async fn serve(config: Config, db: Pool<Sqlite>) -> anyhow::Result<()> {
    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext {
                config: Arc::new(config),
                db,
            }))
            .layer(TraceLayer::new_for_http())
            .layer(middleware::from_fn(telemetry)),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .await
        .map_err(anyhow::Error::from)
}

fn api_router() -> Router {
    let serve_dir = ServeDir::new("assets");
    Router::new()
        .route("/", get(index))
        .route("/repo/:name", get(repo))
        .fallback_service(serve_dir)
}
