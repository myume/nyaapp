use std::{fs::read_dir, io, vec};

use app_lib::metadata::mangabaka::Mangabaka;
use tempdir::TempDir;

#[tokio::test(flavor = "multi_thread")]
async fn test_db_setup() {
    let client = reqwest::Client::new();
    let dir = TempDir::new("test").unwrap();
    Mangabaka::setup(&client, dir.path()).await.unwrap();
    let files = read_dir(dir.path())
        .unwrap()
        .map(|res| res.map(|file| file.file_name().to_string_lossy().into_owned()))
        .collect::<Result<Vec<String>, io::Error>>()
        .unwrap();

    assert_eq!(files, vec!["series.sqlite"]);
}
