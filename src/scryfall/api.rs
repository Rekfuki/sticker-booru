use reqwest;
use reqwest::{RedirectPolicy, StatusCode, Url};

use crate::scryfall::models::SearchResult;

const BASE_URL: &str = "https://api.scryfall.com/";

/// Returns the search result of 'query' in pages of 175 cards.
pub fn cards_search(query: &str, order: &str, page: i32) -> reqwest::Result<SearchResult> {
    unimplemented!()
}
