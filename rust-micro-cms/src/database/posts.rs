use crate::models::{NewPost, Post, Posts, UpdatePost};
use rusqlite::{Connection, Error, Result};


pub async fn create_posts_table(conn: &Connection) -> Result<usize> {
    let sql = 
        "CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            date TEXT NOT NULL,
            body TEXT NOT NULL,
            archived BOOLEAN NOT NULL DEFAULT FALSE,
            draft BOOLEAN NOT NULL DEFAULT TRUE,
            author_id INTEGER NOT NULL,
            FOREIGN KEY (author_id) REFERENCES author(id)
        )";
        let result = conn.execute(sql, ())?;
    Ok(result)
}
pub async fn insert_new_post(conn: &Connection, post: &NewPost) -> Result<Post> {
    let sql = "INSERT INTO posts (title, date, body, author_id) VALUES (?, ?, ?, ?) RETURNING *";

    let stmt = conn.query_row(sql, [
        post.title.to_owned(),
        post.date.to_owned(),
        post.body.to_owned(),
        post.author_id.to_string()        
    ], |row| {
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

    Ok(stmt)
}

pub async fn fetch_all_posts(conn: &Connection) -> Result<Posts> {
    let sql = "SELECT * FROM posts";
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
    let mut posts = Vec::new();
    for post in post_iter {
        posts.push(post?);
    }
    Ok(Posts { posts })
}

pub async fn fetch_post_by_id(conn: &Connection, post_id: i32) -> Result<Option<Post>> {
    let sql = "SELECT * FROM posts WHERE id = $1";
    let stmt = conn.query_row(sql, [post_id], |row| {
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

    Ok(Some(stmt))
}

// pub async fn fetch_all_posts_for_author(conn: &Connection, author_id: i32) -> Result<Posts> {
//     let posts = sqlx::query_as!(
//         Post,
//         "SELECT * FROM posts WHERE author_id = $1 AND archived = FALSE",
//         author_id
//     )
//     .fetch_all(pool)
//     .await?;

//     Ok(Posts { posts })
// }

pub async fn update_post(conn: &Connection, post: &UpdatePost) -> Result<Post> {
    let sql = "UPDATE posts SET title = $1, date = $2, body = $3, archived = $4, draft = $5, author_id = $6 WHERE id = $7 RETURNING *";
    let stmt = conn.query_row(sql, [
        post.title.to_owned(),
        post.date.to_owned(),
        post.body.to_owned(),
        post.archived.to_string(),
        post.draft.to_string(),
        post.author_id.to_string(),
        post.id.to_string()
    ], |row| {
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

    Ok(stmt)
}

pub async fn delete_post(conn: &Connection, post_id: i32) -> Result<bool> {
    let sql = "DELETE FROM posts WHERE id = $1";
    let stmt = conn.execute(sql, [post_id])?;
    if stmt > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn toggle_post_draft(conn: &Connection, post_id: i32) -> Result<()> {
    let current_status = sqlx::query!("SELECT draft FROM posts WHERE id = $1", post_id)
        .fetch_one(pool)
        .await?
        .draft;

    let new_status = !current_status;
    sqlx::query!(
        "UPDATE posts SET draft = $1 WHERE id = $2",
        new_status,
        post_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn toggle_post_active(conn: &Connection, post_id: i32) -> Result<()> {
    let current_status = sqlx::query! {
        "SELECT archived FROM posts WHERE id = $1",
        post_id
    }
    .fetch_one(pool)
    .await?
    .archived;

    let new_status = !current_status;
    sqlx::query!(
        "UPDATE posts SET archived = $1 WHERE id = $2",
        new_status,
        post_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
