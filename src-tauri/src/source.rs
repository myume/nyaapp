use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;

pub mod nyaa;

#[async_trait]
pub trait Source: Send + Sync {
    // NyaaInfo probably needs to be more generic to support other sources
    async fn search(&self, query: &str) -> Result<Vec<nyaa::NyaaInfo>>;

    async fn download(&self, id: &str, file_path: &Path) -> Result<()>;
}
