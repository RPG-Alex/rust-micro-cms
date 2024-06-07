use crate::models::nav::{Nav, Social};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension, Result};

pub async fn create_nav_bar_table(pool: &Pool<SqliteConnectionManager>) -> Result<usize> {
    let conn = pool
    .get()
    .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "CREATE TABLE IF NOT EXISTS posts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        item TEXT NOT NULL,
        content TEXT NOT NULL,
        url TEXT
    )";
    conn.execute(sql, ())
}

pub async fn insert_new_item(pool: &Pool<SqliteConnectionManager>, ) -> Result<Nav> {
    let current_date: DateTime<Utc> = Utc::now();
    let date = current_date.format("%Y-%m-%d %H:%M:%S").to_string();

    let conn = pool
        .get()
        .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "INSERT INTO posts (title, date, body, draft) VALUES (?, ?, ?, ?)";
    conn.execute(
        sql,
        params![post.title, date.to_string(), post.body, post.draft],
    )?;

    let last_id = conn.last_insert_rowid();
    conn.query_row(
        "SELECT id, title, date, body, archived, draft FROM posts WHERE id = ?",
        params![last_id],
        |row| {
            Ok(Post {
                id: row.get(0)?,
                title: row.get(1)?,
                date: row.get(2)?,
                body: row.get(3)?,
                archived: row.get(4)?,
                draft: row.get(5)?,
            })
        },
    )
}

pub async fn fetch_all_posts(pool: &Pool<SqliteConnectionManager>) -> Result<Posts> {
    let conn = pool
        .get()
        .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "SELECT * FROM posts WHERE NOT archived";
    let mut stmt = conn.prepare(sql)?;
    let post_iter = stmt.query_map([], |row| {
        Ok(Post {
            id: row.get(0)?,
            title: row.get(1)?,
            date: row.get(2)?,
            body: row.get(3)?,
            archived: row.get(4)?,
            draft: row.get(5)?,
        })
    })?;

    let mut posts = Posts { posts: Vec::new() };
    for post in post_iter {
        posts.posts.push(post?);
    }

    Ok(posts)
}

pub async fn delete_post(pool: &Pool<SqliteConnectionManager>, post_id: i32) -> Result<bool> {
    let conn = pool
        .get()
        .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "DELETE FROM posts WHERE id = ?";
    let affected_rows = conn.execute(sql, [post_id])?;
    Ok(affected_rows > 0)
}