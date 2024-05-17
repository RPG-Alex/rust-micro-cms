use crate::models::Style;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension, Result};

pub async fn create_style_db(pool: &Pool<SqliteConnectionManager>) -> Result<usize> {
    let conn = pool
        .get()
        .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "CREATE TABLE IF NOT EXISTS styling {
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        styling TEXT NOT NULL
    }";
    conn.execute(sql, ())
}