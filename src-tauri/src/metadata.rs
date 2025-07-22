use anyhow::Result;
use async_trait::async_trait;

pub mod mangabaka;

pub struct Metadata {}

#[async_trait]
pub trait MetadataProvider {
    async fn fetch_metdata(&self, title: &str) -> Result<Metadata>;
}
