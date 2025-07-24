use crate::{
    source::{nyaa::category::NyaaCategory, SourceInfo},
    torrent::TorrentService,
};

use super::{FileSize, Source};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use chrono::DateTime;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::{path::Path, sync::Arc};
use url::Url;

pub struct Nyaa {
    base_url: Url,
    client: reqwest::Client,
    torrent_service: Arc<dyn TorrentService>,
}

mod category;
pub mod query_params;

impl super::FileSize {
    fn from(s: &str, re: &Regex) -> Result<Self> {
        let caps = re.captures(s).context("Failed to parse size")?;
        let size = caps["size"].parse()?;
        match &caps["unit"] {
            "MiB" => Ok(FileSize::MiB(size)),
            "GiB" => Ok(FileSize::GiB(size)),
            unit => Err(anyhow!("Unrecognized file size unit {}", unit)),
        }
    }
}

struct NyaaParseConfig {
    category: Selector,
    title: Selector,
    size: Selector,
    size_regex: Regex,
    timestamp: Selector,
    seeders: Selector,
    leechers: Selector,
    completed: Selector,
}

impl NyaaParseConfig {
    pub fn new() -> Self {
        Self {
            category: Selector::parse("td:nth-child(1) a").unwrap(),
            title: Selector::parse("td:nth-child(2) a:last-child").unwrap(),
            size: Selector::parse("td:nth-child(4)").unwrap(),
            size_regex: Regex::new(r"(?<size>[0-9]+\.[0-9]+) (?<unit>MiB|GiB)").unwrap(),
            timestamp: Selector::parse("td:nth-child(5)").unwrap(),
            seeders: Selector::parse("td:nth-child(6)").unwrap(),
            leechers: Selector::parse("td:nth-child(7)").unwrap(),
            completed: Selector::parse("td:nth-child(8)").unwrap(),
        }
    }
}

impl Nyaa {
    pub fn new(torrent_service: Arc<dyn TorrentService>, client: reqwest::Client) -> Self {
        Self {
            base_url: Url::parse("https://nyaa.si/").unwrap(),
            client,
            torrent_service,
        }
    }

    async fn fetch_page(&self, url: &Url) -> Result<Html> {
        let request = self.client.get(url.as_str());
        let response = request.send().await?;
        let content = response.text().await?;
        Ok(Html::parse_document(&content))
    }

    fn extract_id_from_href(href: &str) -> Result<String> {
        href.split("/")
            .last()
            .map(|id| id.to_owned())
            .context(format!("Missing id on href: {}", href))
    }

    async fn get_title_by_id(&self, id: &str) -> Result<String> {
        let url = self.base_url.join("view/")?.join(id)?;
        let html = self.fetch_page(&url).await?;
        let title_selector = Selector::parse(".panel-title").expect("title selector to be valid");
        Ok(html
            .select(&title_selector)
            .next()
            .context(format!("Missing title for id: {}", id))?
            .text()
            .collect::<String>()
            .trim()
            .to_owned())
    }

    fn parse_row(row: ElementRef, config: &NyaaParseConfig) -> Result<SourceInfo> {
        let category = row
            .select(&config.category)
            .next()
            .context("Missing category for row")?
            .attr("href")
            .context("Category link missing 'href' element")?;

        let title_element = row
            .select(&config.title)
            .next()
            .context("Missing title column for row")?;

        let title = title_element
            .attr("title")
            .context("Missing title for row")?
            .to_owned();

        let id = title_element
            .attr("href")
            .context("Missing id for row")
            .and_then(Nyaa::extract_id_from_href)?;

        let size = row
            .select(&config.size)
            .next()
            .context("Missing size for row")?
            .text()
            .collect::<String>();

        let timestamp = row
            .select(&config.timestamp)
            .next()
            .context("Missing timestamp column for row")?
            .attr("data-timestamp")
            .context("Missing timestamp for row")?
            .parse()?;

        let seeders = row
            .select(&config.seeders)
            .next()
            .context("Missing seeders count for row")?
            .text()
            .collect::<String>()
            .parse()?;

        let leechers = row
            .select(&config.leechers)
            .next()
            .context("Missing leechers count for row")?
            .text()
            .collect::<String>()
            .parse()?;

        let completed = row
            .select(&config.completed)
            .next()
            .context("Missing completed count for row")?
            .text()
            .collect::<String>()
            .parse()?;

        Ok(SourceInfo {
            id,
            category: NyaaCategory::from_query_param(category)?.to_source_category(),
            title,
            size: FileSize::from(&size, &config.size_regex)?,
            timestamp: DateTime::from_timestamp(timestamp, 0).context("Invalid timestamp")?,
            seeders,
            leechers,
            completed,
        })
    }
}

#[async_trait]
impl Source for Nyaa {
    fn normalize_title(&self, title: &str) -> String {
        let tags = r"\[.+\]|\{.+\}|\(.+\)";
        let chapter_volume = r"[a-zA-Z]*\d+(-[a-zA-Z]*\d+)*";
        let allowlist = r#"[^a-zA-Z0-9 .,?!'"-:]+"#;

        let normalization = Regex::new(&format!(r"({tags})|({chapter_volume})|{allowlist}"))
            .expect("title normalization regex to be valid");

        let normalized = normalization
            .replace_all(title, "")
            .to_lowercase()
            .trim()
            .to_owned();

        let multi_space = Regex::new(r"\s+").expect("valid regex for multi space");

        multi_space.replace_all(&normalized, " ").to_string()
    }

    async fn search(&self, query: &str) -> Result<Vec<SourceInfo>> {
        log::info!("Searching for {}", query);
        let mut url = self.base_url.clone();
        url.set_query(Some(query));

        let html = self.fetch_page(&url).await?;

        let selector = Selector::parse("tr").unwrap();
        let rows = html.select(&selector);

        let config = NyaaParseConfig::new();

        log::info!("Parsing Nyaa table rows");
        Ok(rows
            .filter_map(|row| {
                Nyaa::parse_row(row, &config)
                    .map_err(|err| {
                        log::warn!("{}", err);
                        err
                    })
                    .ok()
            })
            .collect())
    }

    async fn download(&self, id: &str, base_dir: &Path) -> Result<()> {
        log::info!("Starting download for {}view/{}", self.base_url, id);

        let url = self
            .base_url
            .join("download/")?
            .join(&format!("{}.torrent", id))?;

        let title = self.get_title_by_id(id).await?;
        let output_dir = base_dir.join(&title);

        self.torrent_service
            .download_torrent(&url, &format!("{}.torrent", title), &output_dir)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::torrent::rqbit_service::MockRqbitService;

    use super::*;

    #[rstest]
    #[case(
        "【OSHI NO KO】 001-166 (2022-2024) (Digital) (Antrill) [Completed]",
        "oshi no ko"
    )]
    #[case(
        "The Apothecary Diaries: Xiaolan's Story 001-003 (2025) (Digital) (Oak)",
        "the apothecary diaries: xiaolan's story"
    )]
    #[case("I've Been Killing Slimes for 300 Years and Maxed Out My Level Spin-off - The Red Dragon Academy for Girls v01-02 (2023-2025) (Digital) (1r0n)", "i've been killing slimes for years and maxed out my level spin-off - the red dragon academy for girls")]
    #[case(
        "My Quiet Blacksmith Life in Another World v05 (2025) (Digital) (Ushi)",
        "my quiet blacksmith life in another world"
    )]
    fn test_normalize_title(#[case] title: &str, #[case] expected: &str) {
        let nyaa = Nyaa::new(Arc::new(MockRqbitService::new()), reqwest::Client::new());
        let actual = nyaa.normalize_title(title);
        assert_eq!(actual, expected);
    }
}
