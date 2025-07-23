use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::path::Path;

pub mod nyaa;

#[derive(Debug, Serialize)]
#[serde(tag = "unit", content = "size")]
pub enum FileSize {
    MiB(f32),
    GiB(f32),
}

#[derive(Debug, Serialize)]
pub enum Category {
    Manga,
}

#[derive(Debug, Serialize)]
pub struct SourceInfo {
    pub id: String,
    pub category: Category,
    pub title: String,
    pub size: FileSize,
    pub timestamp: DateTime<Utc>,
    pub seeders: u32,
    pub leechers: u32,
    pub completed: u32,
}

#[async_trait]
pub trait Source: Send + Sync {
    async fn search(&self, query: &str) -> Result<Vec<SourceInfo>>;

    async fn download(&self, id: &str, file_path: &Path) -> Result<()>;
}
