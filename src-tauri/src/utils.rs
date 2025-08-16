use std::path::Path;

use anyhow::Result;
use flate2::read::GzDecoder;
use futures::StreamExt;
use tokio::{
    fs::{read_dir, File},
    io::AsyncWriteExt,
};

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

pub async fn read_files_from_dir(path: &Path) -> Result<Vec<String>> {
    let mut files = vec![];
    let mut entries = read_dir(path).await?;
    while let Ok(Some(file)) = entries.next_entry().await {
        files.push(file.file_name().to_string_lossy().to_string());
    }

    Ok(files)
}

pub fn parse_pages_uri(uri: &str) -> Result<(String, usize, usize), String> {
    let path = uri.strip_prefix("/").ok_or("Invalid URI")?;

    let parts: Vec<&str> = path.split('/').collect();

    if parts.len() < 3 {
        return Err(
            "URI must contain at least library id, file number, and page number".to_string(),
        );
    }

    let id = parts[0].to_owned();
    let file_num = parts[1].parse().expect("File num must be a valid number");
    let page_num = parts[2].parse().expect("Page num must be a valid number");

    Ok((id, file_num, page_num))
}
