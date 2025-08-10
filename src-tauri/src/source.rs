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
pub struct SourceMedia {
    pub id: String,
    pub category: Category,
    pub title: String,
    pub size: FileSize,
    pub timestamp: DateTime<Utc>,
    pub seeders: u32,
    pub leechers: u32,
    pub completed: u32,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct PaginationInfo {
    pub min_page: u32,
    pub max_page: u32,
    pub has_prev: bool,
    pub has_next: bool,
}

#[async_trait]
pub trait Source: Send + Sync {
    fn normalize_title(&self, title: &str) -> String;

    async fn search(&self, query: &str) -> Result<(Vec<SourceMedia>, PaginationInfo)>;

    async fn download(&self, id: &str, file_path: &Path) -> Result<()>;

    async fn get_title_by_id(&self, id: &str) -> Result<String>;
}
