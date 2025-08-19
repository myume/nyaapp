use std::{collections::HashMap, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_vec};
use tokio::{
    fs::{read_to_string, File},
    io::AsyncWriteExt,
};

use crate::{metadata::Metadata, settings::LibraryEntrySettings, source::SourceMeta};

#[derive(Serialize, Deserialize, Clone)]
pub struct ReadingProgress {
    pub current_page: usize,
    pub total_pages: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Metafile {
    pub source: SourceMeta,
    pub metadata: Option<Metadata>,
    pub reading_progress: HashMap<String, ReadingProgress>,
    pub settings: Option<LibraryEntrySettings>,
}

impl Metafile {
    pub fn new(source: SourceMeta, metadata: Option<Metadata>) -> Self {
        Metafile {
            source,
            metadata,
            reading_progress: HashMap::new(),
            settings: None,
        }
    }

    pub async fn write(&self, output_dir: &Path) -> Result<()> {
        let metafile_path = output_dir.join(".meta");

        let mut file = File::create(&metafile_path).await?;
        file.write_all(to_vec(&self)?.as_slice()).await?;

        log::info!("Successfully wrote metafile to {}", metafile_path.display());

        Ok(())
    }

    pub async fn read(dir: &Path) -> Result<Metafile> {
        let metafile_content = read_to_string(dir.join(".meta")).await?;
        let metafile: Metafile = from_str(&metafile_content)?;
        Ok(metafile)
    }
}
