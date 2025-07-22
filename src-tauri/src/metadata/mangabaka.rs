use std::path::Path;

use crate::utils::{download_file_from_url, unpack_tarball};

use super::Metadata;
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{query_as, sqlite::SqlitePoolOptions, SqlitePool};
use strsim::sorensen_dice;
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
    async fn fetch_metdata(&self, title: &str) -> Result<Metadata> {
        let parts = title
            .split(|c: char| c.is_ascii_punctuation())
            .collect::<Vec<&str>>();

        let pattern = parts.join("%");
        let pattern = format!("%{pattern}%");

        // for some reason all the columns in the provided db is nullable
        // im forcing these columns to be non-null here. if something crashes, it's probably this.
        let rows = query_as!(
            Metadata,
            r#"SELECT
                id as "id!",
                title as "title!",
                cover_default as "cover!",
                authors as "authors!",
                artists as "artists!",
                description as "description!",
                genres as "genres!",
                type as "media_type!",
                year as "year!",
                status as "status!",
                tags as "tags!"
            FROM series WHERE LOWER(title) LIKE LOWER($1) AND merged_with IS NULL"#,
            pattern
        )
        .fetch_all(&self.pool)
        .await?;

        let normalized_title = parts.join(" ").to_lowercase();

        let mut results = rows
            .iter()
            .map(|row| {
                (
                    row,
                    sorensen_dice(&row.title.to_string().to_lowercase(), &normalized_title),
                )
            })
            .collect::<Vec<(&Metadata, f64)>>();
        results.sort_by(|a, b| b.1.total_cmp(&a.1));

        let best_match = results
            .get(0)
            .context(format!("Failed to find metdata for {}", title))?
            .0
            .clone();

        Ok(best_match.clone())
    }
}
