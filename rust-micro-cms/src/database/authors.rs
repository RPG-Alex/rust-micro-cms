use tokio_rusqlite::{Connection, Error, params, Result};
use crate::models::{Author, Authors, NewAuthor, UpdateAuthor};

pub async fn create_author_table(conn: &Connection) -> Result<usize> {
    conn.call(move |conn| {
        let sql = 
            "CREATE TABLE IF NOT EXISTS author (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                deleted BOOLEAN DEFAULT FALSE
            )";
        conn.execute(sql, [])
    }).await
}

pub async fn insert_new_author(conn: &Connection, author: &NewAuthor) -> Result<Author> {
    conn.call(move |conn| {
        let sql = "INSERT INTO author (first_name, last_name) VALUES (?1, ?2)";
        conn.execute(sql, params![author.first_name, author.last_name])?;

        let last_id = conn.last_insert_rowid();
        let mut stmt = conn.prepare("SELECT id, first_name, last_name, deleted FROM author WHERE id = ?1")?;
        let author = stmt.query_row(params![last_id], |row| {
            Ok(Author {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                deleted: row.get(3)?
            })
        })?;
        Ok(author)
    }).await
}

pub async fn fetch_all_authors(conn: &Connection) -> Result<Authors> {
    conn.call(move |conn| {
        let sql = "SELECT id, first_name, last_name, deleted FROM author WHERE deleted = FALSE";
        let mut stmt = conn.prepare(sql)?;
        let authors = stmt.query_map([], |row| {
            Ok(Author {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                deleted: row.get(3)?
            })
        })?
        .map(|res| res.unwrap())
        .collect::<Vec<_>>();
        Ok(Authors { authors })
    }).await
}

pub async fn fetch_author_by_id(conn: &Connection, author_id: i32) -> Result<Option<Author>> {
    conn.call(move |conn| {
        let sql = "SELECT id, first_name, last_name, deleted FROM author WHERE id = ? AND deleted = FALSE";
        let mut stmt = conn.prepare(sql)?;
        stmt.query_row(params![author_id], |row| {
            Ok(Author {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                deleted: row.get(3)?
            })
        })
    }).await
}

pub async fn update_author(conn: &Connection, author: UpdateAuthor) -> Result<Author> {
    conn.call(move |conn| {
        let sql = "UPDATE author SET first_name = ?, last_name = ? WHERE id = ? RETURNING id, first_name, last_name, deleted";
        let mut stmt = conn.prepare(sql)?;
        stmt.query_row(params![author.first_name, author.last_name, author.id], |row| {
            Ok(Author {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                deleted: row.get(3)?
            })
        })
    }).await
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
