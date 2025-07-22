use anyhow::Result;
use async_trait::async_trait;

pub mod mangabaka;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Metadata {
    pub id: i64,
    pub title: String,
    pub cover: String,
    pub authors: String,
    pub artists: String,
    pub description: String,
    pub year: i64,
    pub tags: String,
    pub media_type: String,
    pub status: String,
    pub genres: String,
}

#[async_trait]
pub trait MetadataProvider {
    async fn fetch_metdata(&self, title: &str) -> Result<Metadata>;
}
