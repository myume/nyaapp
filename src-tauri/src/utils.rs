use std::path::Path;

use anyhow::Result;
use flate2::read::GzDecoder;
use futures::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn download_file_from_url(
    client: &reqwest::Client,
    url: &url::Url,
    filename: &str,
    output_dir: &Path,
) -> Result<()> {
    let output_path = output_dir.join(&filename);

    let request = client.get(url.as_str());
    let response = request.send().await?.error_for_status()?;

    let mut file = File::create(output_path).await?;
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        file.write_all(&chunk?).await?;
    }
    Ok(())
}

pub fn unpack_tarball(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref().to_owned();
    let output_dir = path.parent().expect("tarball should have parent dir");

    let tar_gz_file = std::fs::File::open(&path)?;
    let tar = GzDecoder::new(tar_gz_file);
    let mut archive = tar::Archive::new(tar);

    archive.unpack(output_dir)?;

    Ok(())
}
