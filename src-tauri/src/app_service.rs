use anyhow::Result;
use librqbit::Session;
use std::{path::PathBuf, sync::Arc};

use crate::{
    metadata::mangabaka::Mangabaka,
    source::{nyaa::Nyaa, Source},
    torrent::{rqbit_service::RqbitService, TorrentService},
};

pub struct AppService {
    source: Box<dyn Source>,
    base_dir: PathBuf,
    pub torrent_service: Arc<dyn TorrentService>,
    client: reqwest::Client,
    pub mangabaka_provider: Mangabaka,
}

impl AppService {
    pub async fn new(data_dir: PathBuf) -> Result<Self> {
        let session = Session::new(data_dir.clone()).await.unwrap();
        let client = reqwest::Client::new();
        let torrent_service = Arc::new(RqbitService::new(session, client.clone()));

        Ok(AppService {
            source: Box::new(Nyaa::new(torrent_service.clone(), client.clone())),
            mangabaka_provider: Mangabaka::setup(&client, &data_dir.join("db")).await?,
            base_dir: data_dir,
            torrent_service,
            client,
        })
    }
}
