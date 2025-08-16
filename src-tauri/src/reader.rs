use anyhow::Result;
use std::path::Path;

pub mod cbz_reader;

pub trait Reader {
    fn load(&mut self, path: &Path) -> Result<usize>;

    fn get(&self, path: &Path, index: usize) -> Option<Vec<u8>>;

    fn list(&self, path: &Path) -> Option<Vec<Vec<u8>>>;
}
