use super::MetadataProvider;

struct Mangabaka {}

impl MetadataProvider for Mangabaka {
    fn fetch_metdata(title: &str) -> super::Metadata {
        todo!()
    }
}
