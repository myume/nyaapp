use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use log::info;
use serde::Serialize;
use tokio::fs::{read_dir, remove_dir_all};

use crate::{
    metafile::{Metafile, ReadingProgress},
    reader::Reader,
    utils::read_files_from_dir,
};

#[derive(Serialize, Clone)]
pub struct LibraryEntry {
    pub name: String,
    pub metafile: Metafile,
    pub output_dir: PathBuf,
    pub files: Vec<String>,
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
        let name = output_dir
            .file_name()
            .expect("Missing filename")
            .to_string_lossy()
            .to_string();

        log::info!("Adding \"{}\" to library", name);

        self.entries.insert(
            metafile.source.id.clone(),
            LibraryEntry {
                metafile,
                name,
                files: read_files_from_dir(&output_dir).await?,
                output_dir,
            },
        );

        Ok(())
    }
    pub async fn get_entry(&mut self, id: &str) -> Option<LibraryEntry> {
        self.entries.get(id).cloned()
    }

    pub fn get_entries(&self) -> Vec<LibraryEntry> {
        self.entries.values().cloned().collect()
    }

    async fn fetch_library(library_dir: &Path) -> Result<HashMap<String, LibraryEntry>> {
        info!("Fetching library...");
        let mut library = HashMap::new();

        let mut children = read_dir(library_dir).await?;

        while let Ok(Some(dir)) = children.next_entry().await {
            info!("Found: {}", dir.path().display());

            let metafile = Metafile::read(&dir.path()).await?;

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
                },
            );
        }

        info!("Found {} entries in library", library.len());

        Ok(library)
    }

    pub async fn delete(&mut self, id: &str) -> Result<()> {
        let entry = self
            .entries
            .remove(id)
            .context(format!("Missing library entry for {}", id))?;

        remove_dir_all(&entry.output_dir).await?;

        Ok(())
    }

    pub async fn update_reading_progress(
        &mut self,
        id: &str,
        file_num: usize,
        updated_page: usize,
        reader: &mut impl Reader,
    ) -> Result<()> {
        let entry = self
            .entries
            .get_mut(id)
            .context(format!("Missing library entry for {}", id))?;

        let filename = entry
            .files
            .get(file_num)
            .context(format!("No file at index {}", file_num))?;

        if Some(updated_page)
            == entry
                .metafile
                .reading_progress
                .get(filename)
                .map(|progress| progress.current_page)
        {
            log::info!("Reading progress for: {} is up to date", filename,);
            return Ok(());
        }

        if let Some(progress) = entry.metafile.reading_progress.get_mut(filename) {
            progress.current_page = updated_page;
        } else {
            entry.metafile.reading_progress.insert(
                filename.clone(),
                ReadingProgress {
                    current_page: updated_page,
                    // if we are loading the file in the updating_reading_progress function, it
                    // means that the file is has already been loaded so this should be a gauranteed cache
                    // hit. We will not read any files here, just return the number of files.
                    // Kind of a hacky way to acheive this since it's so dependent on the order,
                    // but for now it's a bit more elegant than reading/extracting the cbz files
                    // just to fetch the number of files again.
                    total_pages: reader.load(&entry.output_dir.join(filename))?,
                },
            );
        }

        log::info!(
            "Updating reading progress for: {} to page {}",
            filename,
            updated_page,
        );

        entry.metafile.write(&entry.output_dir).await?;

        Ok(())
    }
}
