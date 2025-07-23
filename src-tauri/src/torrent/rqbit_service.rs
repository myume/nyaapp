use anyhow::Result;
use async_trait::async_trait;
use librqbit::{AddTorrent, AddTorrentOptions};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::fs::create_dir;

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
        output_dir: &Path,
    ) -> Result<()> {
        if !output_dir.exists() {
            create_dir(output_dir).await?;
        }

        let torrent_file_location = self
            .download_torrent_file(file_url, filename, output_dir)
            .await?;

        let mut options = AddTorrentOptions::default();
        options.output_folder = Some(
            output_dir
                .to_str()
                .expect("output dir to be valid")
                .to_owned(),
        );

        let handle = self
            .session
            .add_torrent(
                AddTorrent::from_local_filename(torrent_file_location.to_str().unwrap())?,
                Some(options),
            )
            .await?
            .into_handle()
            .unwrap();

        handle.wait_until_completed().await?;
        log::info!("Download for {} is complete!", filename);
        Ok(())
    }
}
