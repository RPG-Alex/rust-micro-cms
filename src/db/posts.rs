use sqlx::{Pool, Sqlite};
use crate::db::DBConnection;
use crate::models::{Post, Posts};
use anyhow::Result;

impl DBConnection {
    pub async fn create_posts_table(&self) -> Result<()> {
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
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn insert_new_post(&self, post: &Post) -> Result<Post> {
        let inserted_post = sqlx::query_as!(
            Post,
            "INSERT INTO posts (title, date, body, author_id) VALUES (?, ?, ?, ?) RETURNING *",
            post.title, post.date, post.body, post.author_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(inserted_post)
    }
}