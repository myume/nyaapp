use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Result;
use log::info;
use serde::Serialize;
use serde_json::from_str;
use tokio::fs::{read_dir, read_to_string};

use crate::{app_service::Metafile, metadata::Metadata, utils::read_files_from_dir};

#[derive(Serialize, Clone)]
pub struct LibraryEntry {
    pub name: String,
    pub metafile: Metafile,
    pub output_dir: PathBuf,
    pub files: Vec<String>,
    pub metadata: Option<Metadata>,
}

pub struct Library {
    entries: HashMap<String, LibraryEntry>,
}

impl Library {
    pub async fn new(library_dir: &Path) -> Self {
        info!("Initializing Library...");

        Self {
            entries: Library::fetch_library(library_dir)
                .await
                .expect("Failed to read library"),
        }
    }

    pub async fn add_entry(&mut self, metafile: Metafile, output_dir: PathBuf) -> Result<()> {
        self.entries.insert(
            metafile.source.id.clone(),
            LibraryEntry {
                metafile,
                name: output_dir
                    .file_name()
                    .expect("Missing filename")
                    .to_string_lossy()
                    .to_string(),
                files: read_files_from_dir(&output_dir).await?,
                output_dir,
                metadata: None,
            },
        );

        Ok(())
    }

    pub async fn delete_entry(&mut self) {}

    pub fn get_entries(&self) -> Vec<LibraryEntry> {
        self.entries.values().cloned().collect()
    }

    async fn fetch_library(library_dir: &Path) -> Result<HashMap<String, LibraryEntry>> {
        let mut library = HashMap::new();

        let mut children = read_dir(library_dir).await?;

        while let Ok(Some(dir)) = children.next_entry().await {
            info!("Found: {}", dir.path().display());

            let metafile_content = read_to_string(dir.path().join(".meta")).await?;
            let metafile: Metafile = from_str(&metafile_content)?;

            let mut files = read_files_from_dir(&dir.path()).await?;
            files.sort();
            files = files
                .into_iter()
                .filter(|file| !file.ends_with(".torrent") && file != ".meta")
                .collect();

            library.insert(
                metafile.source.id.clone(),
                LibraryEntry {
                    name: dir.file_name().to_string_lossy().to_string(),
                    metafile,
                    output_dir: dir.path(),
                    files,
                    metadata: None,
                },
            );
        }

        Ok(library)
    }
}
