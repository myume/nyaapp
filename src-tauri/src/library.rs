use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use log::info;
use serde::{Deserialize, Serialize};
use tokio::fs::{read_dir, remove_dir_all};

use crate::{
    metadata::Metadata,
    metafile::{Metafile, ReadingProgress},
    reader::Reader,
    settings::ReaderSettings,
    utils::read_files_from_dir,
};

#[derive(Serialize, Clone)]
pub struct LibraryEntry {
    pub name: String,
    pub metafile: Metafile,
    pub output_dir: PathBuf,
    pub files: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LibraryEntrySettings {
    pub reader: ReaderSettings,
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
                files: Library::get_files(&output_dir).await?,
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

    async fn get_files(entry_path: &Path) -> Result<Vec<String>> {
        let mut files = read_files_from_dir(entry_path).await?;
        files.sort();
        Ok(files
            .into_iter()
            .filter(|file| !file.ends_with(".torrent") && file != ".meta")
            .collect())
    }

    pub async fn update_library_entry_title(
        &mut self,
        id: &str,
        title: &str,
        metadata: Option<Metadata>,
    ) -> Result<()> {
        let entry = self
            .entries
            .get_mut(id)
            .context("Could not find library entry with corresponding id")?;

        let new_output_dir = entry.output_dir.with_file_name(title);
        tokio::fs::rename(&entry.output_dir, &new_output_dir).await?;
        entry.output_dir = new_output_dir;

        entry.name = title.to_owned();
        entry.metafile.metadata = metadata;
        entry.metafile.write(&entry.output_dir).await
    }

    async fn fetch_library(library_dir: &Path) -> Result<HashMap<String, LibraryEntry>> {
        info!("Fetching library...");
        let mut library = HashMap::new();

        let mut children = read_dir(library_dir).await?;

        while let Ok(Some(dir)) = children.next_entry().await {
            info!("Found: {}", dir.path().display());

            let Ok(metafile) = Metafile::read(&dir.path()).await else {
                log::error!("Failed to read metadata for {}", dir.path().display());
                continue;
            };

            library.insert(
                metafile.source.id.clone(),
                LibraryEntry {
                    name: dir.file_name().to_string_lossy().to_string(),
                    metafile,
                    output_dir: dir.path(),
                    files: Library::get_files(&dir.path()).await?,
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
        reader: &impl Reader,
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
                    total_pages: reader.num_pages(&entry.output_dir.join(filename))?,
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

    pub async fn update_library_entry_settings(
        &mut self,
        id: &str,
        settings: LibraryEntrySettings,
    ) -> Result<()> {
        let entry = self
            .entries
            .get_mut(id)
            .context(format!("Missing library entry for {}", id))?;

        entry.metafile.settings = Some(settings);
        entry.metafile.write(&entry.output_dir).await?;

        Ok(())
    }

    pub async fn clear_reading_progress(
        &mut self,
        id: &str,
        file_num: Option<usize>,
    ) -> Result<()> {
        let entry = self
            .entries
            .get_mut(id)
            .context(format!("Missing library entry for {}", id))?;

        match file_num {
            Some(file_num) => {
                let filename = entry
                    .files
                    .get(file_num)
                    .context(format!("No file at index {}", file_num))?;

                log::info!("Clearing reading progress for {}", filename);
                entry.metafile.reading_progress.remove(filename);
            }
            None => {
                log::info!("Clearing all reading progress for {}", entry.name);
                entry.metafile.reading_progress.clear();
            }
        }
        entry.metafile.write(&entry.output_dir).await
    }

    pub async fn mark_as_read(
        &mut self,
        id: &str,
        file_num: usize,
        reader: &impl Reader,
    ) -> Result<()> {
        let entry = self
            .entries
            .get_mut(id)
            .context(format!("Missing library entry for {}", id))?;
        let filename = entry
            .files
            .get(file_num)
            .context(format!("No file at index {}", file_num))?;

        log::info!("Marking {} as read", filename);

        let total_pages = reader.num_pages(&entry.output_dir.join(filename))?;
        self.update_reading_progress(id, file_num, total_pages - 1, reader)
            .await
    }
}
