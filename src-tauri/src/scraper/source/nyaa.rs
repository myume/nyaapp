use super::SourceScraper;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::StreamExt;
use librqbit::{AddTorrent, Session};
use std::path::{Path, PathBuf};
use tokio::{fs::File, io::AsyncWriteExt};
use url::Url;

pub struct NyaaScraper {
    base_url: Url,
    client: reqwest::Client,
}

impl NyaaScraper {
    pub fn new() -> Self {
        Self {
            base_url: Url::parse("https://nyaa.si").unwrap(),
            client: reqwest::Client::new(),
        }
    }

    async fn download_torrent(&self, id: &str, base_dir: &Path) -> Result<PathBuf> {
        let filename = format!("{}.torrent", id);
        let output_path = base_dir.join(&filename);

        let path = format!("download/{}", filename);
        let url = self.base_url.join(&path)?;
        log::info!(
            "Downloading torrent file for {} to {}",
            id,
            output_path.to_str().unwrap_or("unknown")
        );

        let request = self.client.get(url.as_str());
        let response = request.send().await?;
        match response.error_for_status() {
            Ok(response) => {
                let mut torrent_file = File::create(base_dir.join(filename)).await?;
                let mut stream = response.bytes_stream();
                while let Some(chunk) = stream.next().await {
                    torrent_file.write_all(&chunk?).await?;
                }
                log::info!(
                    "Finished downloading torrent file for {} to {}",
                    id,
                    base_dir.to_str().unwrap_or("")
                );
                Ok(output_path)
            }
            Err(err) => {
                log::error!("Failed to download torrent file from {}", url);
                Err(anyhow!(err))
            }
        }
    }

    async fn download_torrent_content(&self, base_dir: &Path, torrent: PathBuf) -> Result<()> {
        let session = Session::new(base_dir.to_path_buf()).await.unwrap();
        let filename = torrent.to_str().unwrap();
        let handle = session
            .add_torrent(AddTorrent::from_local_filename(filename)?, None)
            .await?
            .into_handle()
            .unwrap();

        handle.wait_until_completed().await?;
        println!("Download for {} is complete!", filename);
        Ok(())
    }
}

#[async_trait]
impl SourceScraper for NyaaScraper {
    async fn search(&self, key: &str) {
        todo!()
    }

    async fn download(&self, id: &str, base_dir: &Path) -> Result<()> {
        let torrent_file = self.download_torrent(id, base_dir).await?;
        self.download_torrent_content(&base_dir, torrent_file)
            .await?;

        Ok(())
    }
}
