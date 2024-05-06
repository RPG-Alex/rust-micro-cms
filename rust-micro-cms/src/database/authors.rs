use crate::models::{Author, Authors, NewAuthor, UpdateAuthor};
use rusqlite::{Connection, Result};

pub async fn create_author_table(conn: &Connection) -> Result<()> {
    let sql = 
        "CREATE TABLE IF NOT EXISTS author (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            deleted BOOLEAN DEFAULT FALSE
        )";
    conn.execute(sql, ());
    Ok(())
}

pub async fn insert_new_author(conn: &Connection, author: &NewAuthor) -> Result<Author> {
    let sql = "INSERT INTO author (first_name,last_name) VALUES (?,?)";

    conn.execute(sql, [author.first_name.to_owned(),
        author.last_name.to_owned()]).expect("add error handling!");
    
    let last_id = conn.last_insert_rowid();
    let mut stmt = conn.prepare("SELECT id, first_name, last_name FROM author WHERE id = ?1")?;
    let author = stmt.query_row([last_id], |row| {
        Ok(Author {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            deleted: Some(row.get(3)?)
        })
    })?;

    Ok(author)
}

pub async fn fetch_all_authors(conn: &Connection) -> Result<Authors> {
    let sql = "SELECT * FROM author WHERE deleted = FALSE";
    let authors = conn.execute(sql, ());
    
   }

pub async fn fetch_author_by_id(conn: &Connection, author_id: i32) -> Result<Option<Author>> {
    let author = sqlx::query_as!(
        Author,
        "SELECT * FROM author WHERE id = ? AND deleted = false",
        author_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(author)
}

pub async fn update_author(conn: &Connection, new_author: UpdateAuthor) -> Result<()> {
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
pub async fn delete_author(conn: &Connection, author_id: i32) -> Result<()> {
    sqlx::query!("DELETE FROM author WHERE id = ?", author_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn soft_delete_author(conn: &Connection, author_id: i32) -> Result<()> {
    sqlx::query!("UPDATE author SET deleted = TRUE WHERE id = ?", author_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn reactivate_author(conn: &Connection, author_id: i32) -> Result<()> {
    sqlx::query!("UPDATE author SET deleted = FALSE WHERE id = ?", author_id)
        .execute(pool)
        .await?;
    Ok(())
}
