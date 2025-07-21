use anyhow::Result;
use async_trait::async_trait;
use librqbit::AddTorrent;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{torrent::TorrentService, utils::download_file_from_url};

pub struct RqbitService {
    session: Arc<librqbit::Session>,
    client: reqwest::Client,
}

impl RqbitService {
    pub fn new(session: Arc<librqbit::Session>, client: reqwest::Client) -> Self {
        Self { session, client }
    }

    async fn download_torrent_file(
        &self,
        file_url: &url::Url,
        filename: &str,
        base_dir: &Path,
    ) -> Result<PathBuf> {
        let output_path = base_dir.join(&filename);

        log::info!(
            "Downloading torrent file from {} to {}",
            file_url,
            output_path.to_str().unwrap_or("unknown")
        );
        download_file_from_url(&self.client, &file_url, &filename, base_dir)
            .await
            .map_err(|err| {
                log::error!("Failed to download torrent file from {}", &file_url);
                err
            })?;
        log::info!(
            "Finished downloading torrent file from {} to {}",
            file_url,
            base_dir.to_str().unwrap_or("")
        );
        Ok(output_path)
    }
}

#[async_trait]
impl TorrentService for RqbitService {
    async fn download_torrent(
        &self,
        file_url: &url::Url,
        filename: &str,
        base_dir: &Path,
    ) -> Result<()> {
        let torrent_file_location = self
            .download_torrent_file(file_url, filename, base_dir)
            .await?;
        let handle = self
            .session
            .add_torrent(
                AddTorrent::from_local_filename(torrent_file_location.to_str().unwrap())?,
                None,
            )
            .await?
            .into_handle()
            .unwrap();

        handle.wait_until_completed().await?;
        log::info!("Download for {} is complete!", filename);
        Ok(())
    }
}
