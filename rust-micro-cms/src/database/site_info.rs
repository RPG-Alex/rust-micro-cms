use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};

#[derive(Debug)]
pub struct SiteInfo {
    pub id: i64,
    pub site_name: String,
    pub author_name: String,
    pub description: Option<String>,
}