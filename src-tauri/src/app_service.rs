use anyhow::Result;
use librqbit::Session;
use serde::Serialize;
use std::{path::PathBuf, sync::Arc, vec};
use tokio::fs::create_dir;

use crate::{
    metadata::{mangabaka::Mangabaka, Metadata, MetadataProvider},
    source::{nyaa::Nyaa, Category, Source, SourceInfo},
    torrent::{rqbit_service::RqbitService, TorrentService},
};

pub struct AppService {
    source: Box<dyn Source>,
    base_dir: PathBuf,
    pub torrent_service: Arc<dyn TorrentService>,
    pub mangabaka_provider: Arc<Mangabaka>,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub source_info: SourceInfo,
    pub metadata: Metadata,
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
            mangabaka_provider: Arc::new(
                Mangabaka::setup(&client, &app_data_dir.join("db")).await?,
            ),
            base_dir: app_data_dir,
            torrent_service,
        })
    }

    pub async fn download(&self, id: String) -> Result<()> {
        let library_dir = self.base_dir.join("library");
        self.source.download(&id, &library_dir).await
    }

    fn get_metadata_provider_from_category(
        &self,
        category: &Category,
    ) -> Arc<dyn MetadataProvider> {
        match category {
            Category::Manga => self.mangabaka_provider.clone(),
        }
    }

    pub async fn search(&self, query: String) -> Result<Vec<SearchResult>> {
        let source_info = self.source.search(&query).await?;
        let mut results = vec![];
        for source in source_info {
            let metadata_provider = self.get_metadata_provider_from_category(&source.category);
            let metadata = metadata_provider.fetch_metdata(&source.title).await?;
            results.push(SearchResult {
                source_info: source,
                metadata,
            });
        }
        Ok(results)
    }
}
