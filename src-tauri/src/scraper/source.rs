use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;

pub mod nyaa;

#[async_trait]
pub trait SourceScraper {
    async fn search(&self, key: &str);

    async fn download(&self, id: &str, file_path: &Path) -> Result<()>;
}
