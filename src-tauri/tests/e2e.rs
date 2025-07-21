#[cfg(test)]
mod e2e_tests {
    use app_lib::source::{nyaa::Nyaa, Source};
    use app_lib::torrent::rqbit_service::RqbitService;
    use app_lib::torrent::MockTorrentService;
    use librqbit::Session;
    use std::sync::Arc;
    use std::{fs::read_dir, io};
    use tempdir::TempDir;

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn test_e2e_download() {
        let dir = TempDir::new("test").unwrap();
        let mock_torrent_service = Session::new(dir.path().to_path_buf()).await.unwrap();
        let client = reqwest::Client::new();

        let rqbit = RqbitService::new(mock_torrent_service, client.clone());
        let nyaa = Nyaa::new(Arc::new(rqbit), client);

        nyaa.download("1990813", dir.path()).await.unwrap();

        let mut files = read_dir(dir.path())
            .unwrap()
            .map(|res| res.map(|file| file.file_name().to_string_lossy().into_owned()))
            .collect::<Result<Vec<String>, io::Error>>()
            .unwrap();

        files.sort();

        assert_eq!(files.len(), 2);
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn test_e2e_search() {
        let client = reqwest::Client::new();
        let rqbit = MockTorrentService::new();

        let nyaa = Nyaa::new(Arc::new(rqbit), client);
        let results = nyaa.search("c=3_0").await.unwrap();
        assert_eq!(75, results.len());
    }
}
