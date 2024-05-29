use crate::models::styling::{NewStyle, Style};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};

pub async fn create_style_db(pool: &Pool<SqliteConnectionManager>) -> Result<usize> {
    let conn = pool
        .get()
        .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "CREATE TABLE IF NOT EXISTS styles (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        styling TEXT NOT NULL
    )";
    conn.execute(sql, ())
}

pub async fn fetch_all_styles(pool: &Pool<SqliteConnectionManager>) -> Result<Vec<Style>> {
    let conn = pool
        .get()
        .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "SELECT * FROM styles";
    let mut stmt = conn.prepare(sql)?;
    let styling_iter = stmt.query_map([], |row| {
        Ok(Style {
            id: row.get(0)?,
            name: row.get(1)?,
            css: row.get(2)?,
        })
    })?;
    let mut styles = Vec::new();
    for post in styling_iter {
        styles.push(post?);
    }

    Ok(styles)
}

pub async fn insert_style(pool: &Pool<SqliteConnectionManager>, style: NewStyle) -> Result<Style> {
    let conn = pool
        .get()
        .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "INSERT INTO styles (name, styling) VALUES (?, ?)";
    conn.execute(sql, params![style.name, style.css])?;

    let last_id = conn.last_insert_rowid();
    conn.query_row(
        "SELECT * FROM styles WHERE id = ?",
        params![last_id],
        |row| {
            Ok(Style {
                id: row.get(0)?,
                name: row.get(1)?,
                css: row.get(2)?,
            })
        },
    )
}
