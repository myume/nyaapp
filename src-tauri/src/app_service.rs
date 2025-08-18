use anyhow::{Context, Result};
use librqbit::{Session, SessionOptions, SessionPersistenceConfig};
use serde::Serialize;
use std::{path::PathBuf, sync::Arc, vec};
use tokio::{
    fs::create_dir,
    sync::{watch::Receiver, Mutex},
};

use crate::{
    library::{Library, LibraryEntry},
    metadata::{mangabaka::Mangabaka, Metadata, MetadataProvider},
    metafile::Metafile,
    reader::{cbz_reader::CBZReader, Reader},
    source::{nyaa::Nyaa, Category, MediaInfo, PaginationInfo, Source, SourceMeta},
    torrent::{rqbit_service::RqbitService, TorrentService, TorrentStats},
};

pub struct AppService {
    source: Box<dyn Source>,
    base_dir: PathBuf,
    pub torrent_service: Arc<Mutex<dyn TorrentService>>,
    pub mangabaka_provider: Arc<Mangabaka>,
    library: Arc<Mutex<Library>>,
    cbz_reader: Arc<Mutex<CBZReader>>,
}

#[derive(Serialize)]
pub struct SearchResult {
    media_info: MediaInfo,
    metadata: Option<Metadata>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    search_results: Vec<SearchResult>,
    pagination: PaginationInfo,
}

impl AppService {
    pub async fn new(app_data_dir: PathBuf) -> Result<Self> {
        log::info!("Initializing app service");

        let library_dir = app_data_dir.join("library");
        if !library_dir.exists() {
            create_dir(&library_dir).await?;
        }

        let library = Library::new(&library_dir).await;

        let session = Session::new_with_opts(
            library_dir.clone(),
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
            library: Arc::new(Mutex::new(library)),
            cbz_reader: Arc::new(Mutex::new(CBZReader::new())),
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
        let output_dir = self.source.download(id, &library_dir).await?;
        let metafile = Metafile::new(
            SourceMeta {
                id: id.to_owned(),
                provider: self.source.get_variant(),
            },
            self.get_metadata_by_id(id).await.ok(),
        );

        log::info!("Writing metafile for {}", id);
        metafile.write(&output_dir).await?;

        self.library
            .lock()
            .await
            .add_entry(metafile, output_dir)
            .await?;

        Ok(())
    }

    pub async fn get_title_by_id(&self, id: &str) -> Result<String> {
        self.source.get_info_by_id(id).await.map(|info| info.title)
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
        log::info!("Listing torrents");
        self.torrent_service.lock().await.list_torrents()
    }

    pub async fn search(&self, query: String) -> Result<SearchResponse> {
        let (media_info, pagination) = self.source.search(&query).await?;
        let mut results = vec![];

        let mut metadata_hits = 0;

        for media in media_info {
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
                media_info: media,
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

    pub async fn toggle_pause(&self, id: &str) -> Result<()> {
        log::info!("Pausing download for {}", id);
        self.torrent_service.lock().await.toggle_pause(id).await
    }

    async fn get_metadata_by_id(&self, id: &str) -> Result<Metadata> {
        let info = self.source.get_info_by_id(id).await?;
        let metadata_provider = self.get_metadata_provider_from_category(&info.category);
        let normalized_title = self.source.normalize_title(&info.title);
        metadata_provider
            .fetch_metdata(&normalized_title)
            .await
            .map_err(|err| {
                log::warn!(
                    "No metdata found for \"{}\": {}",
                    info.title,
                    err.to_string()
                );
                err
            })
    }

    pub async fn fetch_library(&self) -> Vec<LibraryEntry> {
        log::info!("Fetching library");
        self.library.lock().await.get_entries()
    }

    pub async fn remove_download(&self, id: &str) -> Result<()> {
        log::info!("Removing {} from torrent client", id);
        self.torrent_service.lock().await.remove_torrent(id).await
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        log::info!("Removing {} from torrent client", id);
        self.torrent_service.lock().await.remove_torrent(id).await?;
        log::info!("Removing {} from library", id);
        self.library.lock().await.delete(id).await
    }

    pub async fn load_cbz(&self, id: &str, file_num: usize) -> Result<usize> {
        let entry = self
            .library
            .lock()
            .await
            .get_entry(id)
            .await
            .context(format!("Failed to find entry with id {} in library", id))?;

        let filename = entry.files.get(file_num).context("File not found")?;

        log::info!("Loading files from {}", filename);

        let num_pages = self
            .cbz_reader
            .lock()
            .await
            .load(&entry.output_dir.join(filename))?;

        log::info!("Loaded {} pages from {}", num_pages, filename);

        Ok(num_pages)
    }

    pub async fn get_page(&self, id: &str, file_num: usize, page_num: usize) -> Result<Vec<u8>> {
        let entry = self
            .library
            .lock()
            .await
            .get_entry(id)
            .await
            .context(format!("Failed to find entry with id {} in library", id))?;

        let filename = entry.files.get(file_num).context("File not found")?;

        log::info!("Fetching page {} from {}", page_num, filename);

        self.cbz_reader
            .lock()
            .await
            .get(&entry.output_dir.join(filename), page_num)
            .context(format!("Failed to find page {} for {}", page_num, filename))
    }

    pub async fn update_reading_progress(
        &mut self,
        id: &str,
        file_num: usize,
        updated_page: usize,
    ) -> Result<()> {
        let mut reader = self.cbz_reader.lock().await;
        self.library
            .lock()
            .await
            .update_reading_progress(id, file_num, updated_page, &mut *reader)
            .await
    }

    pub async fn get_dimensions(&self, id: &str, file_num: usize) -> Result<Vec<(u32, u32)>> {
        let entry = self
            .library
            .lock()
            .await
            .get_entry(id)
            .await
            .context(format!("Failed to find entry with id {} in library", id))?;

        let filename = entry.files.get(file_num).context("File not found")?;

        log::info!("Fetching dimensions for {}", filename);

        self.cbz_reader
            .lock()
            .await
            .get_dimensions(&entry.output_dir.join(filename))
            .context(format!("Failed to get dimensions for {}", filename))
    }
}
