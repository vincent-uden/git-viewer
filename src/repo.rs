use std::{fs, path::Path};

use anyhow::anyhow;
use axum::{extract, Extension};
use chrono::{DateTime, Local};
use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::{http::{ApiContext, AppError}, index::footer};

fn render_readme(path: &Path) -> Result<Markup, AppError> {
    if path.exists() {
        let contents = fs::read_to_string(path)?;
        let converted = markdown::to_html(&contents);
        return Ok(html! {
            div class="md-root" {
                (PreEscaped(converted))
            }
        });
    }

    Err(anyhow!("Couldnt render readme").into())
}

fn readme_fallback() -> Markup {
    html! {
        div {
            p { "No readme" }
        }
    }
}

pub async fn repo(Extension(ctx): Extension<ApiContext>, extract::Path(name): extract::Path<String>) -> Result<Markup, AppError> {
    let disk_path = ctx.config.git_root.join(&name).join("README.md");
    let total_visits = sqlx::query!(r#"select count(*) as count from visits;"#)
        .fetch_one(&ctx.db)
        .await?;
    let last_visit =
        sqlx::query!(r#"select created_at from visits order by created_at desc limit 1"#)
            .fetch_one(&ctx.db)
            .await?;
    let timestamp: DateTime<Local> = DateTime::from(
        DateTime::from_timestamp_millis(last_visit.created_at).unwrap_or(DateTime::default()),
    );

    Ok(html! {
        (DOCTYPE)
        link rel="stylesheet" href="/style.css";
        main {
            a href="/" { h1 { "Git Registry" } }
            h2 { (name) }

            p { "Clone the repo" }
            pre { "git clone " (ctx.config.clone_root) (name) }

            h2 { "Readme" }

            ( render_readme(&disk_path).unwrap_or(readme_fallback()) )
            (footer(total_visits.count, timestamp))
        }
    })
}
