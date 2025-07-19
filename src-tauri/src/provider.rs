use anyhow::Result;
use async_trait::async_trait;
use std::path::Path;

pub mod nyaa;

#[async_trait]
pub trait Provider {
    async fn search(&self, query: &str) -> Result<Vec<nyaa::NyaaInfo>>;

    async fn list(&self);

    async fn download(&self, id: &str, file_path: &Path) -> Result<()>;
}
