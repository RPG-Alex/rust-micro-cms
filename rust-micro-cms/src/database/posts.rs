use crate::models::{NewPost, Post, Posts, UpdatePost};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension, Result};

pub async fn create_posts_table(pool: &Pool<SqliteConnectionManager>) -> Result<usize> {
    let conn = pool
        .get()
        .map_err(|e| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            date TEXT NOT NULL,
            body TEXT NOT NULL,
            archived BOOLEAN NOT NULL DEFAULT FALSE,
            draft BOOLEAN NOT NULL DEFAULT TRUE,
            author_id INTEGER NOT NULL,
            FOREIGN KEY (author_id) REFERENCES author(id)
        )";
    conn.execute(sql, ())
}

pub async fn insert_new_post(pool: &Pool<SqliteConnectionManager>, post: &NewPost) -> Result<Post> {
    let conn = pool
        .get()
        .map_err(|e| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "INSERT INTO posts (title, date, body, author_id) VALUES (?, ?, ?, ?)";
    conn.execute(
        sql,
        params![post.title, post.date, post.body, post.author_id],
    )?;

    let last_id = conn.last_insert_rowid();
    conn.query_row(
        "SELECT id, title, date, body, archived, draft, author_id FROM posts WHERE id = ?",
        params![last_id],
        |row| {
            Ok(Post {
                id: row.get(0)?,
                title: row.get(1)?,
                date: row.get(2)?,
                body: row.get(3)?,
                archived: row.get(4)?,
                draft: row.get(5)?,
                author_id: row.get(6)?,
            })
        },
    )
}

pub async fn fetch_all_posts(pool: &Pool<SqliteConnectionManager>) -> Result<Posts> {
    let conn = pool
        .get()
        .map_err(|e| rusqlite::Error::ExecuteReturnedResults)?;
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
            author_id: row.get(6)?,
        })
    })?;

    let mut posts = Posts { posts: Vec::new() };
    for post in post_iter {
        posts.posts.push(post?);
    }

    Ok(posts)
}

pub async fn fetch_post_by_id(
    pool: &Pool<SqliteConnectionManager>,
    post_id: i32,
) -> Result<Option<Post>> {
    let conn = pool
        .get()
        .map_err(|e| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "SELECT * FROM posts WHERE id = ?";
    conn.query_row(sql, params![post_id], |row| {
        Ok(Post {
            id: row.get(0)?,
            title: row.get(1)?,
            date: row.get(2)?,
            body: row.get(3)?,
            archived: row.get(4)?,
            draft: row.get(5)?,
            author_id: row.get(6)?,
        })
    })
    .optional()
}

pub async fn update_post(pool: &Pool<SqliteConnectionManager>, post: UpdatePost) -> Result<Post> {
    let conn = pool
        .get()
        .map_err(|e| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "UPDATE posts SET title = ?, date = ?, body = ?, archived = ?, draft = ?, author_id = ? WHERE id = ? RETURNING *";
    conn.query_row(
        sql,
        params![
            post.title,
            post.date,
            post.body,
            post.archived,
            post.draft,
            post.author_id,
            post.id
        ],
        |row| {
            Ok(Post {
                id: row.get(0)?,
                title: row.get(1)?,
                date: row.get(2)?,
                body: row.get(3)?,
                archived: row.get(4)?,
                draft: row.get(5)?,
                author_id: row.get(6)?,
            })
        },
    )
}

pub async fn delete_post(pool: &Pool<SqliteConnectionManager>, post_id: i32) -> Result<bool> {
    let conn = pool
        .get()
        .map_err(|e| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "DELETE FROM posts WHERE id = ?";
    let affected_rows = conn.execute(sql, [post_id])?;
    Ok(affected_rows > 0)
}

// pub async fn toggle_post_draft(conn: &Connection, post_id: i32) -> Result<()> {
//     let current_status = sqlx::query!("SELECT draft FROM posts WHERE id = $1", post_id)
//         .fetch_one(pool)
//         .await?
//         .draft;

//     let new_status = !current_status;
//     sqlx::query!(
//         "UPDATE posts SET draft = $1 WHERE id = $2",
//         new_status,
//         post_id
//     )
//     .execute(pool)
//     .await?;

//     Ok(())
// }
// pub async fn toggle_post_active(conn: &Connection, post_id: i32) -> Result<()> {
//     let current_status = sqlx::query! {
//         "SELECT archived FROM posts WHERE id = $1",
//         post_id
//     }
//     .fetch_one(pool)
//     .await?
//     .archived;

//     let new_status = !current_status;
//     sqlx::query!(
//         "UPDATE posts SET archived = $1 WHERE id = $2",
//         new_status,
//         post_id
//     )
//     .execute(pool)
//     .await?;

//     Ok(())
// }
