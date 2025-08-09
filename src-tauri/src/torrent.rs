use anyhow::Result;
use serde::Serialize;
use std::path::Path;
use tokio::sync::watch::Receiver;

use async_trait::async_trait;

pub mod rqbit_service;

#[derive(Clone, Serialize)]
pub struct TorrentStats {
    id: String,
    state: String,
    progress_bytes: u64,
    uploaded_bytes: u64,
    total_bytes: u64,
    finished: bool,

    // speed is in mbps
    upload_speed: Option<f64>,
    download_speed: Option<f64>,

    remaining_time: Option<String>,
}

#[async_trait]
pub trait TorrentService: Send + Sync {
    async fn download_torrent(
        &mut self,
        id: &str,
        file_url: &url::Url,
        filename: &str,
        base_dir: &Path,
    ) -> Result<()>;

    async fn wait_until_finished(&mut self, id: &str) -> Result<()>;

    fn get_stats_receiver(&self, id: &str) -> Option<Receiver<TorrentStats>>;

    fn list_torrents(&self) -> Vec<TorrentStats>;
}
