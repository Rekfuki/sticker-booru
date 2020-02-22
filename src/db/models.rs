use diesel_derives::Queryable;
use std::collections::HashSet;

/// Unused fields are omitted
#[derive(Queryable)]
pub struct Sticker {
    pub id: String,
    pub tags: HashSet<String>,
}
