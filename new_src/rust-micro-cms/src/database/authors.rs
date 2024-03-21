use crate::models::{Author, Authors};
use anyhow::Result;
// going to deimplement this as impl for a struct. 
impl DBConnection {
    pub async fn create_author_table(&self) -> Result<()> {
        sqlx::query!(
            "CREATE TABLE IF NOT EXISTS author (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
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
            "INSERT INTO author (first_name,last_name) VALUES (?,?) RETURNING *",
            author.first_name, author.last_name
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

    pub async fn update_author(&self, author_id: i32, new_author_first_name: &str, new_author_last_name: &str) -> Result<()> {
        sqlx::query_as!(
            Author,
            "UPDATE author SET first_name = ?, last_name = ? WHERE id = ?",
            new_author_first_name, new_author_last_name, author_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(())
    }

    //could cause conflict due to key restraints with posts
    pub async fn delete_author(&self, author_id: i32) -> Result<()> {
        sqlx::query!(
            "DELETE FROM author WHERE id = ?",
            author_id
        )
        .execute(&self.pool)
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