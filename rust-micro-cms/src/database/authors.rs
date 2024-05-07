use rusqlite::{params, Result, OptionalExtension};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use crate::models::{Author, Authors, NewAuthor, UpdateAuthor};

pub async fn create_author_table(pool: &Pool<SqliteConnectionManager>) -> Result<usize> {
    let conn = pool.get().map_err(|e| rusqlite::Error::ExecuteReturnedResults)?; // Custom error handling
    let sql = 
        "CREATE TABLE IF NOT EXISTS author (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            deleted BOOLEAN DEFAULT FALSE
        )";
    conn.execute(sql, ())
}

pub async fn insert_new_author(pool: &Pool<SqliteConnectionManager>, author: &NewAuthor) -> Result<Author> {
    let conn = pool.get().map_err(|e| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "INSERT INTO author (first_name, last_name) VALUES (?, ?)";
    conn.execute(sql, params![author.first_name, author.last_name])?;
    
    let last_id = conn.last_insert_rowid();
    conn.query_row("SELECT id, first_name, last_name, deleted FROM author WHERE id = ?", params![last_id], |row| {
        Ok(Author {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            deleted: row.get(3)?
        })
    })
}

pub async fn fetch_all_authors(pool: &Pool<SqliteConnectionManager>) -> Result<Authors> {
    let conn = pool.get().map_err(|e| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "SELECT id, first_name, last_name, deleted FROM author WHERE deleted = FALSE";
    let mut stmt = conn.prepare(sql)?;
    let author_iter = stmt.query_map([], |row| {
        Ok(Author {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            deleted: row.get(3)?
        })
    })?;

    let mut authors = Authors { authors: Vec::new() };
    for author in author_iter {
        authors.authors.push(author?);
    }

    Ok(authors)
}

pub async fn fetch_author_by_id(pool: &Pool<SqliteConnectionManager>, author_id: i32) -> Result<Option<Author>> {
    let conn = pool.get().map_err(|e| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "SELECT id, first_name, last_name, deleted FROM author WHERE id = ? AND deleted = FALSE";
    conn.query_row(sql, params![author_id], |row| {
        Ok(Author {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            deleted: row.get(3)?
        })
    }).optional()
}

pub async fn update_author(pool: &Pool<SqliteConnectionManager>, new_author: UpdateAuthor) -> Result<Author> {
    let conn = pool.get().map_err(|e| rusqlite::Error::ExecuteReturnedResults)?;
    let sql = "UPDATE author SET first_name = ?, last_name = ? WHERE id = ? RETURNING *";
    conn.query_row(sql, params![new_author.first_name, new_author.last_name, new_author.id.to_string()], |row| {
        Ok(Author {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            deleted: row.get(3)?
        })
    })
}

//could cause conflict due to key restraints with posts
// pub async fn delete_author(conn: &Connection, author_id: i32) -> Result<()> {
//     sqlx::query!("DELETE FROM author WHERE id = ?", author_id)
//         .execute(pool)
//         .await?;
//     Ok(())
// }

// pub async fn soft_delete_author(conn: &Connection, author_id: i32) -> Result<()> {
//     sqlx::query!("UPDATE author SET deleted = TRUE WHERE id = ?", author_id)
//         .execute(pool)
//         .await?;
//     Ok(())
// }

// pub async fn reactivate_author(conn: &Connection, author_id: i32) -> Result<()> {
//     sqlx::query!("UPDATE author SET deleted = FALSE WHERE id = ?", author_id)
//         .execute(pool)
//         .await?;
//     Ok(())
// }
