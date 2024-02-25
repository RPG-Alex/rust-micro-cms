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
                deleted BOOLEAN NOT NULL DEFAULT FALSE,
                draft BOOLEAN NOT NULL DEFAULT TRUE,
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

    pub async fn fetch_all_posts(&self) -> Result<Posts> {
        let posts = sqlx::query_as!(
            Post,
            "SELECT * FROM posts"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(Posts { posts })
    }

    pub async fn fetch_post_by_id(&self, post_id: i32) -> Result<Option<Post>> {
        let post = sqlx::query_as!(
            Post,
            "SELECT * FROM posts WHERE id = $1",
            post_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }


    pub async fn fetch_all_posts_for_author(&self, author_id: i32) -> Result<Posts> {
        let posts = sqlx::query_as!(
            Post,
            "SELECT * FROM posts WHERE author_id = $1 AND deleted = FALSE",
            author_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(Posts { posts })
    }

    pub async fn update_post(&self, post_id: i32, post: &Post) -> Result<()> {
        sqlx::query_as!(
            Post,
            "UPDATE posts SET title = $1, date = $2, body = $3, author_id = $4 WHERE id = $5",
            post.title, post.date, post.body, post.author_id, post_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_post(&self, post_id: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM posts WHERE id = $1",
            post_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn toggle_post_draft(&self, post_id: i32) -> Result<()> {
        let current_status = sqlx::query!(
            "SELECT draft FROM posts WHERE id = $1",
            post_id
        )
        .fetch_one(&self.pool)
        .await?
        .draft;

        let new_status = !current_status;
        sqlx::query!(
            "UPDATE posts SET draft = $1 WHERE id = $2",
            new_status, post_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn soft_delete_post(&self, post_id: i32) -> Result<()> {
        sqlx::query!(
            "UPDATE posts SET deleted = TRUE WHERE id = $1",
            post_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn restore_post(&self, post_id: i32) -> Result<()> {
        sqlx::query!(
            "UPDATE posts SET deleted = FALSE WHERE id = $1",
            post_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

}