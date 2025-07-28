use anyhow::{Context, Ok, Result};
use async_trait::async_trait;
use librqbit::{AddTorrent, AddTorrentOptions, ManagedTorrent, TorrentStats as RqBitTorrentStats};

use log::info;
#[cfg(test)]
use mockall::automock;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::{
    fs::create_dir,
    sync::watch::{self, Receiver},
};

use crate::{
    torrent::{TorrentService, TorrentStats},
    utils::download_file_from_url,
};

pub struct RqbitService {
    session: Arc<librqbit::Session>,
    client: reqwest::Client,
    handles: HashMap<String, Arc<ManagedTorrent>>,
    receivers: HashMap<String, Receiver<TorrentStats>>,
}

impl RqbitService {
    pub fn new(session: Arc<librqbit::Session>, client: reqwest::Client) -> Self {
        Self {
            session,
            client,
            handles: HashMap::new(),
            receivers: HashMap::new(),
        }
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

    fn to_stats(id: String, stats: RqBitTorrentStats) -> TorrentStats {
        TorrentStats {
            id,
            state: stats.state.to_string(),
            progress_bytes: stats.progress_bytes,
            uploaded_bytes: stats.uploaded_bytes,
            total_bytes: stats.total_bytes,
            finished: stats.finished,
        }
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
impl TorrentService for RqbitService {
    async fn download_torrent(
        &mut self,
        id: &str,
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

        let (tx, rx) = watch::channel(Self::to_stats(id.to_owned(), handle.stats()));
        self.receivers.insert(id.to_owned(), rx);

        tokio::spawn({
            let h = handle.clone();
            let id = id.to_owned();
            async move {
                while !h.stats().finished {
                    let stats = h.stats();
                    info!("{}", stats);
                    tx.send(Self::to_stats(id.to_owned(), stats))?;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                tx.send(Self::to_stats(id.to_owned(), h.stats()))?;
                info!("{}", h.stats());
                Ok(())
            }
        });

        self.handles.insert(id.to_owned(), handle);

        Ok(())
    }

    async fn wait_until_finished(&mut self, id: &str) -> Result<()> {
        let handle = self
            .handles
            .get(id)
            .context(format!("No download with id {}", id))?;

        handle.wait_until_completed().await?;
        log::info!("Download for {} is complete!", id);

        self.handles.remove(id);
        self.receivers.remove(id);

        Ok(())
    }

    fn get_stats_receiver(&self, id: &str) -> Option<Receiver<TorrentStats>> {
        self.receivers.get(id).cloned()
    }
}
