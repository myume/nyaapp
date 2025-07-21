use anyhow::Result;
use std::path::Path;

use async_trait::async_trait;

use mockall::{automock, predicate::*};

pub mod rqbit_service;

#[automock]
#[async_trait]
pub trait TorrentService: Send + Sync {
    async fn download_torrent(
        &self,
        file_url: &url::Url,
        filename: &str,
        base_dir: &Path,
    ) -> Result<()>;
}
