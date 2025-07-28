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
    id_translation: HashMap<usize, String>, // torrent id to source id
}

impl RqbitService {
    pub fn new(session: Arc<librqbit::Session>, client: reqwest::Client) -> Self {
        Self {
            session,
            client,
            handles: HashMap::new(),
            receivers: HashMap::new(),
            id_translation: HashMap::new(),
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
        source_id: &str,
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

        self.id_translation
            .insert(handle.id(), source_id.to_owned());

        let (tx, rx) = watch::channel(Self::to_stats(source_id.to_owned(), handle.stats()));
        self.receivers.insert(source_id.to_owned(), rx);

        tokio::spawn({
            let h = handle.clone();
            let id = source_id.to_owned();
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

        self.handles.insert(source_id.to_owned(), handle);

        Ok(())
    }

    async fn wait_until_finished(&mut self, source_id: &str) -> Result<()> {
        let handle = self
            .handles
            .get(source_id)
            .context(format!("No download with id {}", source_id))?;

        handle.wait_until_completed().await?;
        log::info!("Download for {} is complete!", source_id);

        self.id_translation.remove(&handle.id());
        self.handles.remove(source_id);
        self.receivers.remove(source_id);

        Ok(())
    }

    fn get_stats_receiver(&self, source_id: &str) -> Option<Receiver<TorrentStats>> {
        self.receivers.get(source_id).cloned()
    }

    fn list_torrents(&self) -> Vec<TorrentStats> {
        self.session.with_torrents(|torrents| {
            torrents
                .map(|(id, torrent)| {
                    Self::to_stats(
                        self.id_translation
                            .get(&id)
                            .expect("Torrent to have a source id")
                            .to_owned(),
                        torrent.stats(),
                    )
                })
                .collect::<Vec<TorrentStats>>()
        })
    }
}
