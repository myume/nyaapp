use super::category::{LiteratureSubCategory, NyaaCategory};

trait QueryParam {
    fn to_query_param(&self) -> String;
}

pub enum NyaaFilter {
    NoFilter,
    NoRemakes,
    TrustedOnly,
}

impl QueryParam for NyaaCategory {
    fn to_query_param(&self) -> String {
        match self {
            Self::Literature(subcategory) => format!("3_{}", subcategory.to_query_param()),
        }
    }
}

impl QueryParam for LiteratureSubCategory {
    fn to_query_param(&self) -> String {
        match self {
            Self::Base => String::from("0"),
            Self::EnglishTranslated => String::from("1"),
            Self::NonEnglishTranslated => String::from("2"),
            Self::Raw => String::from("3"),
        }
    }
}

impl QueryParam for NyaaFilter {
    fn to_query_param(&self) -> String {
        match self {
            Self::NoFilter => String::from("0"),
            Self::NoRemakes => String::from("1"),
            Self::TrustedOnly => String::from("2"),
        }
    }
}
