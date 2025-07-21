use crate::torrent::TorrentService;

use super::Source;
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::{path::Path, str::from_utf8, sync::Arc};
use url::Url;

pub struct Nyaa {
    base_url: Url,
    client: reqwest::Client,
    torrent_service: Arc<dyn TorrentService>,
}

mod category;
pub mod query_params;

#[derive(Debug)]
enum FileSize {
    MiB(f32),
    GiB(f32),
}

impl FileSize {
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

#[derive(Debug)]
pub struct NyaaInfo {
    id: String,
    category: category::NyaaCategory,
    title: String,
    size: FileSize,
    timestamp: DateTime<Utc>,
    seeders: u32,
    leechers: u32,
    completed: u32,
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
            base_url: Url::parse("https://nyaa.si").unwrap(),
            client,
            torrent_service,
        }
    }

    fn extract_id_from_href(href: &str) -> Result<String> {
        href.split("/")
            .last()
            .map(|id| id.to_owned())
            .context(format!("Missing id on href: {}", href))
    }

    fn parse_row(row: ElementRef, config: &NyaaParseConfig) -> Result<NyaaInfo> {
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

        Ok(NyaaInfo {
            id,
            category: category::NyaaCategory::from_query_param(category)?,
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
    async fn search(&self, query: &str) -> Result<Vec<NyaaInfo>> {
        let mut url = self.base_url.clone();
        url.set_query(Some(query));

        let request = self.client.get(url.as_str());
        let response = request.send().await?;
        let content = response.bytes().await?;
        let html = Html::parse_document(
            from_utf8(&content[..]).context("Failed to parse Nyaa site: invalid utf8 found.")?,
        );

        let selector = Selector::parse("tr").unwrap();
        let rows = html.select(&selector);

        let config = NyaaParseConfig::new();

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
        let filename = format!("{}.torrent", id);
        let url = self.base_url.join(&format!("download/{}", filename))?;

        self.torrent_service
            .download_torrent(&url, &filename, base_dir)
            .await
    }
}
