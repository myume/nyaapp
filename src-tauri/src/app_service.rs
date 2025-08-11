use anyhow::{Context, Result};
use librqbit::{Session, SessionOptions, SessionPersistenceConfig};
use serde::{Deserialize, Serialize};
use serde_json::to_vec;
use std::{path::PathBuf, sync::Arc, vec};
use tokio::{
    fs::{create_dir, File},
    io::AsyncWriteExt,
    sync::{watch::Receiver, Mutex},
};

use crate::{
    metadata::{mangabaka::Mangabaka, Metadata, MetadataProvider},
    source::{nyaa::Nyaa, Category, PaginationInfo, Source, SourceMedia, SourceMeta},
    torrent::{rqbit_service::RqbitService, TorrentService, TorrentStats},
};

pub struct AppService {
    source: Box<dyn Source>,
    base_dir: PathBuf,
    pub torrent_service: Arc<Mutex<dyn TorrentService>>,
    pub mangabaka_provider: Arc<Mangabaka>,
}

#[derive(Serialize)]
pub struct SearchResult {
    source_media: SourceMedia,
    metadata: Option<Metadata>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    search_results: Vec<SearchResult>,
    pagination: PaginationInfo,
}

#[derive(Serialize, Deserialize)]
pub struct Metafile {
    pub source: SourceMeta,
}

impl AppService {
    pub async fn new(app_data_dir: PathBuf) -> Result<Self> {
        log::info!("Initializing app service");

        let library_dir = app_data_dir.join("library");
        if !library_dir.exists() {
            create_dir(&library_dir).await?;
        }

        let session = Session::new_with_opts(
            library_dir,
            SessionOptions {
                persistence: Some(SessionPersistenceConfig::Json {
                    folder: Some(app_data_dir.clone()),
                }),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let client = reqwest::Client::new();
        let torrent_service = Arc::new(Mutex::new(
            RqbitService::new(session, client.clone(), &app_data_dir.join("session.json")).await,
        ));

        Ok(AppService {
            source: Box::new(Nyaa::new(torrent_service.clone(), client.clone())),
            mangabaka_provider: Arc::new(
                Mangabaka::setup(&client, &app_data_dir.join("db")).await?,
            ),
            base_dir: app_data_dir,
            torrent_service,
        })
    }

    pub async fn get_torrent_stats_receiver(&self, id: &str) -> Result<Receiver<TorrentStats>> {
        self.torrent_service
            .lock()
            .await
            .get_stats_receiver(id)
            .context("Receiver does not exist")
    }

    pub async fn download(&self, id: &str) -> Result<()> {
        let library_dir = self.base_dir.join("library");
        let output_dir = self.source.download(id, &library_dir).await;
        let mut metafile = File::create(output_dir?.join(".meta")).await?;
        let meta = Metafile {
            source: SourceMeta {
                id: id.to_owned(),
                provider: self.source.get_variant(),
            },
        };

        metafile.write_all(to_vec(&meta)?.as_slice()).await?;

        Ok(())
    }

    pub async fn get_title_by_id(&self, id: &str) -> Result<String> {
        self.source.get_title_by_id(id).await
    }

    fn get_metadata_provider_from_category(
        &self,
        category: &Category,
    ) -> Arc<dyn MetadataProvider> {
        match category {
            Category::Manga => self.mangabaka_provider.clone(),
        }
    }

    pub async fn list_torrents(&self) -> Vec<TorrentStats> {
        self.torrent_service.lock().await.list_torrents()
    }

    pub async fn search(&self, query: String) -> Result<SearchResponse> {
        let (source_media, pagination) = self.source.search(&query).await?;
        let mut results = vec![];

        let mut metadata_hits = 0;

        for media in source_media {
            let metadata_provider = self.get_metadata_provider_from_category(&media.category);
            let normalized_title = self.source.normalize_title(&media.title);
            let metadata = metadata_provider
                .fetch_metdata(&normalized_title)
                .await
                .map_err(|err| {
                    log::warn!(
                        "No metdata found for \"{}\": {}",
                        media.title,
                        err.to_string()
                    );
                    err
                })
                .ok();

            if metadata.is_some() {
                metadata_hits += 1;
            }

            results.push(SearchResult {
                source_media: media,
                metadata,
            });
        }

        log::info!(
            "Metadata hit rate: {}/{} = {:.2}%",
            metadata_hits,
            results.len(),
            (metadata_hits as f64 / results.len() as f64 * 100.0)
        );

        Ok(SearchResponse {
            search_results: results,
            pagination,
        })
    }
}
