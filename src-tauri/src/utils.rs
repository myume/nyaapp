use std::path::Path;

use anyhow::Result;
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
