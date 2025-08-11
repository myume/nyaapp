use crate::{
    source::{nyaa::category::NyaaCategory, MediaInfo, PaginationInfo, Sources},
    torrent::TorrentService,
};

use super::{FileSize, Source};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use chrono::DateTime;
use regex::Regex;
use scraper::{CaseSensitivity, ElementRef, Html, Selector};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::Mutex;
use url::Url;

pub struct Nyaa {
    base_url: Url,
    client: reqwest::Client,
    torrent_service: Arc<Mutex<dyn TorrentService>>,
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
    pub fn new(torrent_service: Arc<Mutex<dyn TorrentService>>, client: reqwest::Client) -> Self {
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

    fn parse_row(row: ElementRef, config: &NyaaParseConfig) -> Result<MediaInfo> {
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

        Ok(MediaInfo {
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

    fn get_pagination_info(html: &Html) -> Result<PaginationInfo> {
        let pagination_selector = Selector::parse(".pagination li").unwrap();
        let pagination = html.select(&pagination_selector);

        let page_numbers: Vec<u32> = pagination
            .map(|page| page.text().filter_map(|c| c.trim().parse::<u32>().ok()))
            .flatten()
            .collect();

        let has_prev = html
            .select(&Selector::parse(".pagination li:first-child").unwrap())
            .next()
            .map(|prev| {
                !prev
                    .value()
                    .has_class("disabled", CaseSensitivity::AsciiCaseInsensitive)
            })
            .unwrap_or(false);

        let has_next = html
            .select(&Selector::parse(".pagination li:last-child").unwrap())
            .next()
            .map(|next| {
                !next
                    .value()
                    .has_class("disabled", CaseSensitivity::AsciiCaseInsensitive)
            })
            .unwrap_or(false);

        Ok(PaginationInfo {
            min_page: page_numbers.iter().min().unwrap_or(&1).to_owned(),
            max_page: page_numbers.iter().max().unwrap_or(&1).to_owned(),
            has_prev,
            has_next,
        })
    }
}

#[async_trait]
impl Source for Nyaa {
    fn normalize_title(&self, title: &str) -> String {
        let alternate_titles: Vec<&str> = title.split(['|', '/']).collect();
        let title = if alternate_titles.len() > 1 {
            alternate_titles[1]
        } else {
            alternate_titles[0]
        };
        let title = title.to_lowercase().replace("’", "'");

        let heuristics = [
            r"\[.+\]|\{.+\}|\(.+\)",               // tags and stuff thats bracketed
            r"[a-zA-Z]*\d+(\s?-\s?[a-zA-Z]*\d+)*", // chapter and volume numbers
            r#"[^a-zA-Z0-9 .,?!'-:]+"#,            // allowlist of characters
            r#"\bvolume[.s]?\b|\bchapter[.s]?\b|\bch\.?\b|\bvol\.?\b"#, //blocklist of characters
        ];

        let normalization =
            Regex::new(&heuristics.join("|")).expect("title normalization regex to be valid");

        let normalized = normalization.replace_all(&title, "").trim().to_owned();
        let normalized = normalized.strip_suffix("as").unwrap_or(&normalized);

        let multi_space = Regex::new(r"\s+").expect("valid regex for multi space");

        multi_space.replace_all(&normalized, " ").to_string()
    }

    async fn search(&self, query: &str) -> Result<(Vec<MediaInfo>, PaginationInfo)> {
        log::info!("Searching for {}", query);
        let mut url = self.base_url.clone();
        url.set_query(Some(query));

        let html = self.fetch_page(&url).await?;

        let selector = Selector::parse("tr").unwrap();
        let rows = html.select(&selector);

        let config = NyaaParseConfig::new();

        log::info!("Parsing Nyaa table rows");

        let media_info = rows
            .filter_map(|row| {
                Nyaa::parse_row(row, &config)
                    .map_err(|err| {
                        log::warn!("{}", err);
                        err
                    })
                    .ok()
            })
            .collect();

        Ok((media_info, Nyaa::get_pagination_info(&html)?))
    }

    async fn download(&self, id: &str, base_dir: &Path) -> Result<PathBuf> {
        log::info!("Starting download for {}view/{}", self.base_url, id);

        let url = self
            .base_url
            .join("download/")?
            .join(&format!("{}.torrent", id))?;

        let title = self.get_info_by_id(id).await?.title;
        let output_dir = base_dir.join(&title);

        self.torrent_service
            .lock()
            .await
            .download_torrent(id, &url, &format!("{}.torrent", title), &output_dir)
            .await?;

        Ok(output_dir)
    }

    async fn get_info_by_id(&self, id: &str) -> Result<MediaInfo> {
        let url = self.base_url.join("view/")?.join(id)?;
        let html = self.fetch_page(&url).await?;
        let title_selector = Selector::parse(".panel-title").expect("title selector to be valid");

        let title = html
            .select(&title_selector)
            .next()
            .context(format!("Missing title for id: {}", id))?
            .text()
            .collect::<String>()
            .trim()
            .to_owned();

        let timestamp = html
            .select(
                &Selector::parse("div.row:nth-child(1) > div:nth-child(4)")
                    .expect("valid timestamp selector"),
            )
            .next()
            .context("Missing timestamp")?
            .attr("data-timestamp")
            .context("Missing timestamp")?
            .parse()
            .context("Invalid timestamp")?;

        let cols = html
            .select(&Selector::parse(".panel-body .col-md-5").expect("invalid body selector"))
            .map(|element| element.text().collect())
            .collect::<Vec<String>>();

        let config = NyaaParseConfig::new();

        Ok(MediaInfo {
            id: id.to_owned(),
            category: NyaaCategory::from_str(cols.get(0).expect("Missing category"))
                .to_source_category(),
            title,
            size: FileSize::from(cols.get(6).expect("Missing file size"), &config.size_regex)?,
            timestamp: DateTime::from_timestamp(timestamp, 0).context("Invalid timestamp")?,
            seeders: cols
                .get(3)
                .expect("Missing seederes")
                .parse()
                .context("failed to parse seeders")?,
            leechers: cols
                .get(5)
                .expect("Missing seederes")
                .parse()
                .context("failed to parse leechers")?,
            completed: cols
                .get(7)
                .expect("Missing seederes")
                .parse()
                .context("failed to parse completed")?,
        })
    }

    fn get_variant(&self) -> Sources {
        Sources::Nyaa
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
    #[case("I've Been Killing Slimes for 300 Years and Maxed Out My Level Spin-off - The Red Dragon Academy for Girls v01-02 (2023-2025) (Digital) (1r0n)",
        "i've been killing slimes for years and maxed out my level spin-off - the red dragon academy for girls")]
    #[case(
        "My Quiet Blacksmith Life in Another World v05 (2025) (Digital) (Ushi)",
        "my quiet blacksmith life in another world"
    )]
    #[case(
        "I’m the Evil Lord of an Intergalactic Empire! Vol 02 (Audiobook) [Troglodyte]", // stupid apostrophe
        "i’m the evil lord of an intergalactic empire!"
    )]
    fn test_normalize_title(#[case] title: &str, #[case] expected: &str) {
        let nyaa = Nyaa::new(
            Arc::new(Mutex::new(MockRqbitService::new())),
            reqwest::Client::new(),
        );
        let actual = nyaa.normalize_title(title);
        assert_eq!(actual, expected);
    }
}
