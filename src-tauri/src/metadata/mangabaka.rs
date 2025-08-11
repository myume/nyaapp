use std::path::Path;

use super::Metadata;
use crate::utils::{download_file_from_url, unpack_tarball};
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{query, query_as, sqlite::SqlitePoolOptions, SqlitePool};
use strsim::sorensen_dice;
use tokio::fs::{create_dir, remove_file};
use url::Url;

use super::MetadataProvider;

const MANGABAKA_URL: &str = "https://api.mangabaka.dev/v1/"; // the ending slash is important here, otherwise  it won't join properly

pub struct Mangabaka {
    pool: SqlitePool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MangabakaMetadata {
    pub id: i64,
    pub title: String,
    pub cover: Option<String>,
    pub authors: Option<String>,
    pub artists: Option<String>,
    pub description: Option<String>,
    pub year: Option<i64>,
    pub tags: Option<String>,
    pub media_type: String,
    pub status: String,
    pub genres: Option<String>,
}

impl MangabakaMetadata {
    pub fn to_metadata(self) -> Metadata {
        Metadata {
            id: self.id,
            title: self.title,
            cover: self.cover,
            authors: self.authors.map(|authors| {
                serde_json::from_str(&authors)
                    .expect(&format!("authors to be a json array - found: {}", authors))
            }),
            artists: self.artists.map(|artists| {
                serde_json::from_str(&artists)
                    .expect(&format!("artists to be a json array - found: {}", artists))
            }),
            description: self.description,
            year: self.year,
            tags: self.tags.map(|tags| {
                serde_json::from_str(&tags)
                    .expect(&format!("tags to be a json array - found: {}", tags))
            }),
            media_type: self.media_type,
            status: self.status,
            genres: self.genres.map(|genres| {
                serde_json::from_str(&genres)
                    .expect(&format!("genres to be a json array - found: {}", genres))
            }),
        }
    }
}

impl Mangabaka {
    pub async fn setup(client: &reqwest::Client, output_dir: &Path) -> Result<Self> {
        let db_filename = "series.sqlite";
        let db_path = output_dir.join(db_filename);
        if !output_dir.exists() {
            create_dir(output_dir).await?;
        }

        let has_db = db_path.exists();

        if !has_db {
            Mangabaka::download_db(client, output_dir).await?;
        }

        let db_url = format!("sqlite:{}", db_path.display());
        let pool = Mangabaka::connect_to_db(&db_url).await?;

        if !has_db {
            log::info!("Initializing FTS5 table");
            query(
                r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS series_fts USING fts5(
                title, 
                native_title, 
                romanized_title, 
                secondary_titles_en, 
                content='series',
                tokenize = "unicode61 separators '0123456789'"
            )
            "#,
            )
            .execute(&pool)
            .await?;

            query(r#"INSERT INTO series_fts(series_fts) VALUES('rebuild')"#)
                .execute(&pool)
                .await?;
        }

        Ok(Mangabaka::new(pool))
    }

    async fn download_db(client: &reqwest::Client, output_dir: &Path) -> Result<()> {
        let filename = "series.sqlite.tar.gz";
        let download_url = Url::parse(MANGABAKA_URL)?
            .join("database/")?
            .join(filename)?;

        log::info!("Downloading Mangabaka db to {}", output_dir.display());

        download_file_from_url(client, &download_url, filename, output_dir).await?;

        log::info!("Unpacking {} to {}", filename, output_dir.display());

        let tarball = output_dir.join(filename);
        unpack_tarball(&tarball)?;

        log::info!("Cleaning up tarball");
        remove_file(tarball).await?;

        Ok(())
    }

    async fn connect_to_db(db_url: &str) -> Result<SqlitePool> {
        log::info!("Connecting to mangabaka db");
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
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
        log::info!("Fetching metadata for \"{}\" from mangabaka db", title);

        // have to do this to allow punctuation in the title.
        // allowing punctuation in the title to make my life easier because then i don't need to
        // worry about how to split and handle those chars. It's not that much work but here i
        // don't even need to worry about it.
        let fts5_query_string = format!("\"{}\"", title);

        // to figure out nullability i pretty much just manually checked each column
        let rows = query_as!(
            MangabakaMetadata,
            r#"SELECT
                series.id as "id!",
                series.title as "title!",
                cover_default as "cover",
                authors as "authors",
                artists as "artists",
                description as "description",
                genres as "genres",
                type as "media_type!",
                year as "year",
                status as "status!",
                tags as "tags"
            FROM series_fts 
            JOIN series ON series_fts.rowid = series.id
            WHERE series_fts MATCH $1 AND merged_with IS NULL"#,
            fts5_query_string
        )
        .fetch_all(&self.pool)
        .await?;

        let mut results = rows
            .iter()
            .map(|row| {
                (
                    row,
                    sorensen_dice(&row.title.to_string().to_lowercase(), &title),
                )
            })
            .collect::<Vec<(&MangabakaMetadata, f64)>>();
        results.sort_by(|a, b| b.1.total_cmp(&a.1));

        let best_match = results
            .get(0)
            .context(format!("Missing metdata for \"{}\"", title))?
            .0
            .clone()
            .to_metadata();

        Ok(best_match)
    }
}
