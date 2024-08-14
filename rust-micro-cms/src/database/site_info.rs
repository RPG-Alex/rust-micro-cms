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

pub async fn create_site_info_table(pool: &Pool<SqliteConnectionManager>) -> Result<usize> {
    let conn = pool
    .get()
    .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "CREATE TABLE IF NOT EXISTS posts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        site_name TEXT NOT NULL,
        author_name TEXT NOT NULL,
        description TEXT
    )";
    conn.execute(sql, ())
}
/*
TODO:
    Implement the CRUD for the site info
    
*/