use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};
use crate::models::{SiteInfo, SiteName, AuthorName, Description};


pub async fn create_site_info_table(pool: &Pool<SqliteConnectionManager>) -> Result<usize> {
    let conn = pool
    .get()
    .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "CREATE TABLE IF NOT EXISTS site_info (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        site_name TEXT NOT NULL,
        author_name TEXT NOT NULL,
        description TEXT
    )";
    conn.execute(sql, ())
}

pub async fn fetch_site_info(pool: &Pool<SqliteConnectionManager>) -> Result<SiteInfo> {
    let conn = pool
        .get()
        .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "SELECT * from site_info";
    let mut stmt = conn.prepare(sql)?;
    //Modify this later so that it only ever returns one row. There should never be more than one row. Reinforce that here.
    let info_iter = stmt.query_map([], |row| {
        Ok(SiteInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            author_name: row.get(2)?,
            description: row.get(3)?,
        })
    })?;

    // need to extract the first row. 
}