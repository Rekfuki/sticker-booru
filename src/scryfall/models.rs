use serde::Deserialize;
use std::collections::HashSet;

/// Some less important fields are omitted
#[derive(Deserialize)]
pub struct SearchResult {
    pub total_cards: Option<i32>,
    pub has_more: Option<bool>,
    pub data: Option<Vec<Sticker>>,
}

/// Unused fields are omitted
#[derive(Deserialize)]
pub struct Sticker {
    pub id: String,
    pub tags: HashSet<String>,
}
