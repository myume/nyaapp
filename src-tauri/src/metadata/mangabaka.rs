use std::path::Path;

use crate::utils::{download_file_from_url, unpack_tarball};

use super::Metadata;
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tokio::fs::remove_file;
use url::Url;

use super::MetadataProvider;

pub struct Mangabaka {
    pool: SqlitePool,
}

const MANGABAKA_URL: &str = "https://api.mangabaka.dev/v1/"; // the ending slash is important here, otherwise  it won't join properly

impl Mangabaka {
    pub async fn setup(client: &reqwest::Client, output_dir: &Path) -> Result<Self> {
        Mangabaka::download_db(client, output_dir).await?;
        let pool = Mangabaka::connect_to_db(output_dir).await?;
        Ok(Mangabaka::new(pool))
    }

    async fn download_db(client: &reqwest::Client, output_dir: &Path) -> Result<()> {
        let filename = "series.sqlite.tar.gz";
        let download_url = Url::parse(MANGABAKA_URL)?
            .join("database/")?
            .join(filename)?;

        log::info!(
            "Downloading Mangabaka db to {}",
            output_dir.to_str().unwrap_or("unknown dir")
        );

        download_file_from_url(client, &download_url, filename, output_dir).await?;

        log::info!(
            "Unpacking {} to {}",
            filename,
            output_dir.to_str().unwrap_or("unknown dir")
        );

        let tarball = output_dir.join(filename);
        unpack_tarball(&tarball)?;

        log::info!("Cleaning up tarball");
        remove_file(tarball).await?;

        Ok(())
    }

    async fn connect_to_db(db_dir: &Path) -> Result<SqlitePool> {
        let db_url = format!(
            "sqlite:{}/series.sqlite",
            db_dir.to_str().context("Invalid DB dir")?
        );

        SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .context("Failed to connect to mangabaka db.")
    }

    pub fn new(pool: SqlitePool) -> Self {
        Mangabaka { pool }
    }
}

#[async_trait]
impl MetadataProvider for Mangabaka {
    async fn fetch_metdata(&self, title: &str) -> Metadata {
        todo!()
    }
}
