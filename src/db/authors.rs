/*
Todo:
    - add column for author role
    - add table for author roles?
    - revise author name to: author first and last names
*/

use crate::db::DBConnection;
use crate::models::{Author, Authors};
use anyhow::Result;

impl DBConnection {
    pub async fn create_author_table(&self) -> Result<()> {
        sqlx::query!(
            "CREATE TABLE IF NOT EXISTS author (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                deleted BOOLEAN DEFAULT FALSE
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

    pub async fn fetch_all_authors(&self) -> Result<Authors> {
        let authors = sqlx::query_as!(
            Author,
            "SELECT * FROM author WHERE deleted = FALSE"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(Authors{authors})
    }

    pub async fn fetch_author_by_id(&self, author_id: i32) -> Result<Option<Author>> {
        let author = sqlx::query_as!(
            Author,
            "SELECT * FROM author WHERE id = ? AND deleted = false",
            author_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(author)
    }

    pub async fn update_author(&self, author_id: i32, new_author: &str) -> Result<()> {
        sqlx::query_as!(
            Author,
            "UPDATE author SET name = ? WHERE id = ?",
            new_author, author_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn soft_delete_author(&self, author_id: i32) -> Result<()> {
        sqlx::query!(
            "UPDATE author SET deleted = TRUE WHERE id = ?",
            author_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn reactivate_author(&self, author_id: i32) -> Result<()> {
        sqlx::query!(
            "UPDATE author SET deleted = FALSE WHERE id = ?",
            author_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    

}