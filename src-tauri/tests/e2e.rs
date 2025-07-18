use app_lib::provider::{nyaa::Nyaa, Provider};
use std::{fs::read_dir, io};
use tempdir::TempDir;

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_e2e_download() {
    let nyaa = Nyaa::new();
    let dir = TempDir::new("test").unwrap();
    nyaa.download("1990813", dir.path()).await.unwrap();

    let mut files = read_dir(dir.path())
        .unwrap()
        .map(|res| res.map(|file| file.file_name().to_string_lossy().into_owned()))
        .collect::<Result<Vec<String>, io::Error>>()
        .unwrap();

    files.sort();

    assert_eq!(files.len(), 2);
}
