use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum ReaderLayout {
    LongStrip,
    SinglePage,
    DoublePage,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReaderSettings {
    pub gap: Option<u32>,
    pub background_color: Option<String>,
    pub layout: Option<ReaderLayout>,
}
