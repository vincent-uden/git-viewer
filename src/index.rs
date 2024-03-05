use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Error;
use axum::Extension;
use maud::{html, Markup};

use crate::http::{ApiContext, AppError};

struct RepoEntry {
    local_path: PathBuf,
    name: String,
}

fn get_repos_from_disk(root: &Path) -> anyhow::Result<Vec<RepoEntry>> {
    let paths = fs::read_dir(root)?;
    let mut out = vec![];
    for p in paths {
        let path = p?.path();
        if fs::metadata(&path)?.is_dir() {
            match path.clone().file_name() {
                Some(name) => {
                    out.push(RepoEntry {
                        local_path: path,
                        name: name.to_string_lossy().to_string(),
                    });
                }
                None => {}
            }
        }
    }
    Ok(out)
}

pub async fn index(Extension(ctx): Extension<ApiContext>) -> Result<Markup, AppError> {
    let repos = get_repos_from_disk(&ctx.config.git_root)?;

    Ok(html! {
        link rel="stylesheet" href="style.css";
        main {
            h1 { "Git Registry" }
            @for repo in &repos {
                p { (repo.name) }
            }
        }
    })
}
