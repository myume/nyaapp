use std::str::FromStr;

use anyhow::{anyhow, Context, Ok, Result};
use strum_macros::EnumString;

#[derive(Debug)]
pub enum NyaaCategory {
    // AllCategories,
    // Anime,
    // Audio,
    Literature(LiteratureSubCategory),
    // LiveAction,
    // Pictures,
    // Software,
}

#[derive(Debug, EnumString)]
pub enum LiteratureSubCategory {
    #[strum(serialize = "0")]
    Base,

    #[strum(serialize = "1")]
    EnglishTranslated,

    #[strum(serialize = "2")]
    NonEnglishTranslated,

    #[strum(serialize = "3")]
    Raw,
}

impl NyaaCategory {
    /// Parse category from the nyaa query param format
    /// where `s` is in the format of `[major_category]_[subcategory]`
    pub fn from_query_param(s: &str) -> Result<Self> {
        let (main, sub) = s
            .strip_prefix("/?c=")
            .context(format!("Invalid query string: {}", s))?
            .split_once("_")
            .context(format!("Invalid category query param found: {}", s))?;

        match main {
            "3" => Ok(NyaaCategory::Literature(
                LiteratureSubCategory::from_str(sub)
                    .context(format!("unhandled subcategory: {}", sub))?,
            )),
            _ => Err(anyhow!("unhandled main category: {}", main)),
        }
    }
}
