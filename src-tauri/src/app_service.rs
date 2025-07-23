use anyhow::Result;
use librqbit::Session;
use std::{path::PathBuf, sync::Arc};
use tokio::fs::create_dir;

use crate::{
    metadata::mangabaka::Mangabaka,
    source::{nyaa::Nyaa, Source},
    torrent::{rqbit_service::RqbitService, TorrentService},
};

pub struct AppService {
    source: Box<dyn Source>,
    base_dir: PathBuf,
    pub torrent_service: Arc<dyn TorrentService>,
    pub mangabaka_provider: Mangabaka,
}

impl AppService {
    pub async fn new(app_data_dir: PathBuf) -> Result<Self> {
        log::info!("Initializing app service");

        let library_dir = app_data_dir.join("library");
        if !library_dir.exists() {
            create_dir(&library_dir).await?;
        }

        let session = Session::new(library_dir).await.unwrap();
        let client = reqwest::Client::new();
        let torrent_service = Arc::new(RqbitService::new(session, client.clone()));

        Ok(AppService {
            source: Box::new(Nyaa::new(torrent_service.clone(), client.clone())),
            mangabaka_provider: Mangabaka::setup(&client, &app_data_dir.join("db")).await?,
            base_dir: app_data_dir,
            torrent_service,
        })
    }

    pub async fn download(&self, id: String) -> Result<()> {
        let library_dir = self.base_dir.join("library");
        self.source.download(&id, &library_dir).await
    }
}
