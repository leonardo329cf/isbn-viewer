use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub authors: Vec<String>,
    pub synopsis: Option<String>,
    pub publisher: Option<String>,
    pub dimensions: Option<Dimensions>,
    pub year: Option<u32>,
    pub format: Option<String>,
    pub page_count: Option<u32>,
    pub subjects: Vec<String>,
    pub location: Option<String>,
    pub retail_price: Option<String>,
    pub cover_url: Option<String>,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
    pub unit: Unit,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Unit {
    CENTIMETER,
}