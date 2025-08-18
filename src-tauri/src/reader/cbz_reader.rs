use anyhow::{Context, Result};
use image::ImageReader;
use std::{
    collections::HashMap,
    io::{Cursor, Read},
    path::Path,
};

use crate::reader::Reader;

type Pages = Vec<Vec<u8>>;

pub struct CBZReader {
    books: HashMap<String, Pages>,
}

impl CBZReader {
    pub fn new() -> Self {
        CBZReader {
            books: HashMap::new(),
        }
    }
}

impl Reader for CBZReader {
    fn load(&mut self, path: &Path) -> Result<usize> {
        let key = path.to_string_lossy().to_string();
        if let Some(pages) = self.books.get(&key) {
            log::info!("Cache hit: {} pages found in cache", pages.len());
            return Ok(pages.len());
        }

        let file = std::fs::File::open(path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        let mut pages = Vec::new();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let mut content = Vec::new();
            file.read_to_end(&mut content)?;
            pages.push(content);
        }
        self.books.insert(key, pages);

        Ok(archive.len())
    }

    fn get(&self, path: &Path, index: usize) -> Option<Vec<u8>> {
        self.books
            .get(&path.to_string_lossy().to_string())?
            .get(index)
            .cloned()
    }

    fn list(&self, path: &Path) -> Option<Pages> {
        Some(self.books.get(&path.to_string_lossy().to_string())?.clone())
    }

    fn get_dimensions(&self, path: &Path) -> Result<Vec<(u32, u32)>> {
        self.books
            .get(&path.to_string_lossy().to_string())
            .context(format!("Unable to find book at {}", path.display()))?
            .into_iter()
            .map(|page_data| {
                let cursor = Cursor::new(page_data);
                ImageReader::new(cursor)
                    .with_guessed_format()
                    .map(|r| r.into_dimensions().unwrap_or((0, 0)))
                    .context("Failed to load image dimensions")
            })
            .collect()
    }
}
