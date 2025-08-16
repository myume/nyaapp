use anyhow::Result;
use std::{collections::HashMap, io::Read, path::Path};

use crate::reader::Reader;

type Pages = HashMap<usize, Vec<u8>>;

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

        let mut map = HashMap::new();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let mut content = Vec::new();
            file.read_to_end(&mut content)?;
            map.insert(i, content);
        }
        self.books.insert(key, map);

        Ok(archive.len())
    }

    fn get(&self, path: &Path, index: usize) -> Option<Vec<u8>> {
        self.books
            .get(&path.to_string_lossy().to_string())?
            .get(&index)
            .cloned()
    }

    fn list(&self, path: &Path) -> Option<Vec<Vec<u8>>> {
        Some(
            self.books
                .get(&path.to_string_lossy().to_string())?
                .values()
                .cloned()
                .collect(),
        )
    }
}
