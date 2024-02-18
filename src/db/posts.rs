use sqlx::{Pool, Sqlite};
use crate::models::{Post, Posts};
use anyhow::Result;

pub async fn create_posts_table(pool: &Pool<Sqlite>) -> Result<()> {
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            date TEXT NOT NULL,
            body TEXT NOT NULL,
            author_id INTEGER NOT NULL,
            FOREIGN KEY (author_id) REFERENCES author(id)
        )"
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_new_post(pool: &Pool<Sqlite>, post: &Post) -> Result<Post> {
    let mut tx = pool.begin().await?;
    let inserted_post = sqlx::query_as!(
        Post,
        "INSERT INTO posts (title, date, body, author_id) VALUES (?, ?, ?, ?) RETURNING *",
        post.title, post.date, post.body, post.author_id
    )
    .fetch_one(&mut tx)
    .await?;

    tx.commit().await?;
    Ok(inserted_post)
}

pub async fn fetch_all_posts(pool: &Pool<Sqlite>) -> Result<Posts> {
    let posts = sqlx::query_as!(
        Post,
        "SELECT id, title, date, body, author_id FROM posts"
    )
    .fetch_all(pool)
    .await?;

    Ok(Posts { posts })
}

pub async fn fetch_single_post(pool: &Pool<Sqlite>, post_id: i32) -> Result<Option<Post>> {
    let post = sqlx::query_as!(
        Post,
        "SELECT id, title, date, body, author_id FROM posts WHERE id = ?",
        post_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(post)
}

pub async fn update_post(pool: &Pool<Sqlite>, post_id: i32, post: &Post) -> Result<Option<Post>> {
    let mut tx = pool.begin().await?;
    let updated_post = sqlx::query_as!(
        Post,
        "UPDATE posts SET title = ?, date = ?, body = ?, author_id = ? WHERE id = ? RETURNING *",
        post.title, post.date, post.body, post.author_id, post_id
    )
    .fetch_optional(&mut tx)
    .await?;

    tx.commit().await?;
    Ok(updated_post)
}

pub async fn delete_post(pool: &Pool<Sqlite>, post_id: i32) -> Result<u64> {
    let mut tx = pool.begin().await?;
    let deleted = sqlx::query!(
        "DELETE FROM posts WHERE id = ?",
        post_id
    )
    .execute(&mut tx)
    .await?
    .rows_affected();

    tx.commit().await?;
    Ok(deleted)
}
