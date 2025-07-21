use std::path::Path;

use super::Metadata;
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tauri::path::BaseDirectory;

use super::MetadataProvider;

pub struct Mangabaka {
    pool: SqlitePool,
}

impl Mangabaka {
    pub async fn setup() -> Result<Self> {
        todo!()
    }
    async fn download_db(output_dir: &Path) -> Result<()> {
        todo!()
    }

    async fn connect_to_db() -> Result<SqlitePool> {
        let db_url = format!(
            "sqlite:{}/db/mangabaka.sqlite",
            BaseDirectory::AppData.variable()
        );

        SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .context("Failed to connect to mangabaka db.")
    }

    pub async fn new(pool: SqlitePool) -> Self {
        Mangabaka { pool }
    }
}

#[async_trait]
impl MetadataProvider for Mangabaka {
    async fn fetch_metdata(&self, title: &str) -> Metadata {
        todo!()
    }
}
