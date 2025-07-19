pub mod mangabaka;

pub struct Metadata {}

pub trait MetadataProvider {
    fn fetch_metdata(title: &str) -> Metadata;
}
