use std::{env, fs::read_dir, io, vec};

use app_lib::metadata::{mangabaka::Mangabaka, MetadataProvider};
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use tempdir::TempDir;

#[tokio::test(flavor = "multi_thread")]
#[ignore]
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

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_fetch_metadata() {
    dotenv().ok();
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let mangabaka = Mangabaka::new(pool);
    let fetched = mangabaka.fetch_metdata("chainsaw man").await.unwrap();
    assert_eq!(fetched.id, 1677);
}
