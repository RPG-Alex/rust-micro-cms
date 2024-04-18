use crate::models::{Author, Authors, NewAuthor, UpdateAuthor};
use anyhow::Result;
use sqlx::sqlite::SqlitePool;

pub async fn create_author_table(pool: &SqlitePool) -> Result<()> {
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS author (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            deleted BOOLEAN DEFAULT FALSE
        )"
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_new_author(pool: &SqlitePool, author: &NewAuthor) -> Result<Author> {
    let inserted_author = sqlx::query!(
        "INSERT INTO author (first_name,last_name) VALUES (?,?) RETURNING *",
        author.first_name,
        author.last_name
    )
    .await?;

    Ok(inserted_author)
}

pub async fn fetch_all_authors(pool: &SqlitePool) -> Result<Authors> {
    let authors = sqlx::query_as!(Author, "SELECT * FROM author WHERE deleted = FALSE")
        .fetch_all(pool)
        .await?;

    Ok(Authors { authors })
}

pub async fn fetch_author_by_id(pool: &SqlitePool, author_id: i32) -> Result<Option<Author>> {
    let author = sqlx::query_as!(
        Author,
        "SELECT * FROM author WHERE id = ? AND deleted = false",
        author_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(author)
}

pub async fn update_author(pool: &SqlitePool, new_author: UpdateAuthor) -> Result<()> {
    sqlx::query_as!(
        UpdateAuthor,
        "UPDATE author SET first_name = ?, last_name = ? WHERE id = ?",
        new_author.first_name,
        new_author.last_name,
        new_author.id
    )
    .fetch_optional(pool)
    .await?;
    Ok(())
}

//could cause conflict due to key restraints with posts
pub async fn delete_author(pool: &SqlitePool, author_id: i32) -> Result<()> {
    sqlx::query!("DELETE FROM author WHERE id = ?", author_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn soft_delete_author(pool: &SqlitePool, author_id: i32) -> Result<()> {
    sqlx::query!("UPDATE author SET deleted = TRUE WHERE id = ?", author_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn reactivate_author(pool: &SqlitePool, author_id: i32) -> Result<()> {
    sqlx::query!("UPDATE author SET deleted = FALSE WHERE id = ?", author_id)
        .execute(pool)
        .await?;
    Ok(())
}
