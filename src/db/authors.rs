use sqlx::{Pool, Transaction, Sqlite};
use crate::db::DBConnection;
use crate::models::{Author, Authors};
use anyhow::Result;

impl DBConnection {
    pub async fn create_author_table(&self) -> Result<()> {
        sqlx::query!(
            "CREATE TABLE IF NOT EXISTS author (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            )"
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_new_author(&self, author: &Author) -> Result<Author> {
        let inserted_author = sqlx::query_as!(
            Author,
            "INSERT INTO author (name) VALUES (?) RETURNING *",
            author.name
        )
        .fetch_one(&self.pool) 
        .await?;

        Ok(inserted_author)
    }


    pub async fn update_author(&self, author_id: i32, new_author: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE author SET name = ? WHERE id = ?",
            new_author, author_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn fetch_all_authors(&self) -> Result<Vec<Author>> {
        let authors = sqlx::query_as!(
            Author,
            "SELECT id, name FROM author"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(authors)
    }

    pub async fn fetch_author_by_id(&self, author_id: i32) -> Result<Option<Author>> {
        let author = sqlx::query_as!(
            Author,
            "SELECT id, name FROM author WHERE id = ?",
            author_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(author)
    }
}