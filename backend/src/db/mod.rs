use rusqlite::{Connection, Result};
use std::path::Path;

//Post Structure for Database
#[derive(Debug)]
pub struct Post {
    pub id: usize,
    pub title: String,
    pub date: String,
    pub body: String,
}

// Structure for vector of posts (such as fetching all from DB)
#[derive(Debug)]
pub struct Posts {
    pub posts: Vec<Post>,
}

// connect to our sqlite db
pub fn establish_connection(db_path: &Path) -> Result<Connection> {
    Connection::open(db_path)
}

// create the psots table if it doesn't exist
pub fn create_posts_table(conn: &Connection) -> Result<()> {
    let sql = "
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            date TEXT NOT NULL,
            body TEXT NOT NULL
        )
    ";
    conn.execute(sql, ())?;
    Ok(())
}

// Get all the posts
pub fn fetch_all_posts(conn: &Connection) -> Result<Posts> {
    let mut stmt = conn.prepare("SELECT id, title, date, body FROM posts")?;
    let post_iter = stmt.query_map((), |row| {
        Ok(Post {
            id: row.get(0)?,
            title: row.get(1)?,
            date: row.get(2)?,
            body: row.get(3)?,
        })
    })?;

    let posts: Result<Vec<Post>> = post_iter.collect();
    let posts = posts?;
    Ok (Posts { posts})
}

// Add a new post
pub fn insert_post(conn: &Connection, title: &str, date:&str, body: &str) -> Result<usize> {
    let sql = "
        INSERT INTO posts (title, date, body)
        VALUES (?1, ?2, ?3)
        ";
    conn.execute(sql, &[title, date, body])
}

//Update a post data
pub fn update_post(conn: &Connection, post_id:usize, new_title:&str, new_date: &str, new_body: &str) -> Result<usize>{
    // convert the usize to a string for query
    let post_id_str = post_id.to_string();
    // update entry
    let sql = "UPDATE posts SET title = ?1, date = ?2, body = ?3 WHERE id = ?4";
    conn.execute(sql, &[new_title,new_date,new_body,&post_id_str])
}

//Delete a post
pub fn delete_post(conn: &Connection, post_id:usize) -> Result<usize> {
    // convert the usize to a string for query
    let post_id_str = post_id.to_string();
    // update entry
    let sql = "DELETE FROM posts WHERE id = ?1";
    // post_id_str is converted to str here for query
    conn.execute(sql, &[&post_id_str])
}