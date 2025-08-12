use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod mangabaka;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub id: i64,
    pub title: String,
    pub cover: Option<String>,
    pub authors: Option<Vec<String>>,
    pub artists: Option<Vec<String>>,
    pub description: Option<String>,
    pub year: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub media_type: String,
    pub status: String,
    pub genres: Option<Vec<String>>,
}

#[async_trait]
pub trait MetadataProvider: Send + Sync {
    /// Fetches associated metadata for the title
    /// Assumes that title is normalized
    async fn fetch_metdata(&self, title: &str) -> Result<Metadata>;
}
