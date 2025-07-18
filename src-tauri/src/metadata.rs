pub struct Metadata {}

pub trait MetadataScraper {
    fn fetch_metdata() -> Metadata;
}
