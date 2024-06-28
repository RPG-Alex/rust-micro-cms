use crate::models::nav::{Nav, NavItem, NavItemType, NewNavItem};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Result};

pub async fn create_nav_bar_table(pool: &Pool<SqliteConnectionManager>) -> Result<usize> {
    let conn = pool
        .get()
        .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "CREATE TABLE IF NOT EXISTS nav (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        item TEXT NOT NULL,
        content TEXT NOT NULL,
        url TEXT
    )";
    conn.execute(sql, ())
}

pub async fn insert_new_item(
    pool: &Pool<SqliteConnectionManager>,
    new_nav_item: NewNavItem,
) -> Result<NavItem> {
    let conn = pool
        .get()
        .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "INSERT INTO nav (item, content, url) VALUES (?, ?, ?)";
    conn.execute(
        sql,
        params![
            NavItemType::as_str(&new_nav_item.item_type),
            new_nav_item.content,
            new_nav_item.url
        ],
    )?;

    let last_id = conn.last_insert_rowid();
    conn.query_row("SELECT * from nav WHERE id = ?", params![last_id], |row| {
        let item_type_str: String = row.get(1)?;
        let item_type = NavItemType::from_str(&item_type_str);

        Ok(NavItem {
            id: row.get(0)?,
            item_type,
            content: row.get(2)?,
            url: row.get(3)?,
        })
    })
}

pub async fn fetch_all_nav_items(pool: &Pool<SqliteConnectionManager>) -> Result<Nav> {
    let conn = pool
        .get()
        .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "SELECT id, item, content, url FROM nav";
    let mut stmt = conn.prepare(sql)?;
    let nav_item_iter = stmt.query_map([], |row| {
        let item_type_str: String = row.get(1)?;
        let item_type = NavItemType::from_str(&item_type_str);

        Ok(NavItem {
            id: row.get(0)?,
            item_type,
            content: row.get(2)?,
            url: row.get(3)?,
        })
    })?;

    let mut nav = Nav { items: Vec::new() };
    for item in nav_item_iter {
        nav.items.push(item?);
    }

    Ok(nav)
}

pub async fn delete_nav_item(pool: &Pool<SqliteConnectionManager>, item_id: i32) -> Result<bool> {
    let conn = pool
        .get()
        .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "DELETE FROM nav WHERE id = ?";
    let affected_rows = conn.execute(sql, [item_id])?;
    Ok(affected_rows > 0)
}
