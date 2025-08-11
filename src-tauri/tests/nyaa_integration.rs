use anyhow::Result;
use app_lib::source::PaginationInfo;
use app_lib::source::{nyaa::Nyaa, Source};
use app_lib::torrent::rqbit_service::RqbitService;
use app_lib::torrent::TorrentService;
use async_trait::async_trait;
use librqbit::Session;
use std::sync::Arc;
use std::{fs::read_dir, io};
use tempdir::TempDir;
use tokio::sync::Mutex;

use mockall::mock;

mock! {
    pub TorrentService {}
    #[async_trait]
    impl TorrentService for TorrentService {
    async fn download_torrent(
        &mut self,
        id: &str,
        file_url: &url::Url,
        filename: &str,
        base_dir: &std::path::Path,
    ) -> Result<()>;

    async fn wait_until_finished(&mut self, id: &str) -> Result<()>;

    fn get_stats_receiver(&self, id: &str) -> Option<tokio::sync::watch::Receiver<app_lib::torrent::TorrentStats> >;

    fn list_torrents(&self) -> Vec<app_lib::torrent::TorrentStats>;
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_e2e_download() {
    let dir = TempDir::new("test").unwrap();
    let session = Session::new(dir.path().to_path_buf()).await.unwrap();
    let client = reqwest::Client::new();

    let rqbit = Arc::new(Mutex::new(
        RqbitService::new(session, client.clone(), dir.path()).await,
    ));
    let nyaa = Nyaa::new(rqbit.clone(), client);

    nyaa.download("1990813", dir.path()).await.unwrap();
    rqbit
        .lock()
        .await
        .wait_until_finished("1990813")
        .await
        .unwrap();

    let library = read_dir(dir.path())
        .unwrap()
        .map(|res| res.map(|file| file.file_name().to_string_lossy().into_owned()))
        .collect::<Result<Vec<String>, io::Error>>()
        .unwrap();

    // library dir should have a new dir for the downloaded series
    assert_eq!(library.len(), 1);

    let series_dir = library.get(0).unwrap();
    let downloaded_files = read_dir(dir.path().join(series_dir))
        .unwrap()
        .map(|res| res.map(|file| file.file_name().to_string_lossy().into_owned()))
        .collect::<Result<Vec<String>, io::Error>>()
        .unwrap();

    assert_eq!(downloaded_files.len(), 2);
}

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_e2e_search() {
    let client = reqwest::Client::new();
    let rqbit = MockTorrentService::new();

    let nyaa = Nyaa::new(Arc::new(Mutex::new(rqbit)), client);
    let (results, pagination) = nyaa.search("c=3_0").await.unwrap();
    assert_eq!(75, results.len());
    assert_eq!(
        pagination,
        PaginationInfo {
            min_page: 1,
            max_page: 6,
            has_prev: false,
            has_next: true,
        }
    )
}
