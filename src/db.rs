use rusqlite::{Connection, Result};
use std::path::Path;
use serde::Serialize;
//Need to use database pool
use r2d2_sqlite::SqliteConnectionManager;


//Post Structure for Database
#[derive(Clone, Debug, Serialize)]
pub struct Post {
    pub id: usize,
    pub title: String,
    pub date: String,
    pub body: String,
    pub author_id: usize,
    pub author: String,
}

// Defining user structure
#[derive(Clone, Debug, Serialize)]
pub struct Author {
    pub author_id: usize,
    pub author: String,
}


// Structure for vector of posts (such as fetching all from DB)
#[derive(Clone, Debug, Serialize)]
pub struct Posts {
    pub posts: Vec<Post>,
}

// connect to our sqlite db
pub fn establish_connection(db_path: &Path) -> Result<r2d2::Pool<SqliteConnectionManager>, r2d2::Error> {
    let manager = SqliteConnectionManager::file(db_path);
    r2d2::Pool::new(manager).map_err(|e| e.into())
}



pub fn create_author_table(conn: &Connection) -> Result<()> {
    let sql = "
        CREATE TABLE IF NOT EXISTS author (
            id INTEGER PRIMARY KEY NOT NULL,
            author TEXT NOT NULL
        )
    ";
    conn.execute(sql, ())?;
    Ok(())
}

pub fn add_author(conn: &Connection, author: &str) -> Result<usize> {
    let sql = "
        INSERT INTO author (author)
        VALUES (?1)
        ";
    conn.execute(sql, &[author])
}

pub fn get_author_info(conn: &Connection, author_id: usize) -> Result<usize> {
    let sql = "SELECT * FROM author WHERE id = ?1";
    conn.execute(sql, &[&author_id])
}

// create the psots table if it doesn't exist
pub fn create_posts_table(conn: &Connection) -> Result<()> {
    let sql = "
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            date TEXT NOT NULL,
            body TEXT NOT NULL,
            author_id INTEGER NOT NULL, 
            FOREIGN KEY (author_id) REFERENCES author(id) 
        )
    ";
    conn.execute(sql, ())?;
    Ok(())
}

// Get all the posts
pub fn fetch_all_posts(conn: &Connection) -> Result<Posts> {
    let mut stmt = conn.prepare("SELECT posts.id, posts.title, posts.date, posts.body, posts.author_id, author.author FROM posts INNER JOIN author ON posts.author_id = author.id")?;
    let post_iter = stmt.query_map((), |row| {
        Ok(Post {
            id: row.get(0)?,
            title: row.get(1)?,
            date: row.get(2)?,
            body: row.get(3)?,
            author_id: row.get(4)?,
            author: row.get(5)?,
        })
    })?;

    let posts: Result<Vec<Post>> = post_iter.collect();
    let posts = posts?;
    Ok (Posts { posts})
}

//fetch a single post by id
pub fn fetch_single_post(conn: &Connection, post_id: usize) -> Result<Option<Post>> {
    let mut stmt = conn.prepare("SELECT posts.id, posts.title, posts.date, posts.body, posts.author_id, author.author FROM posts INNER JOIN author ON posts.author_id = author.id WHERE posts.id = ?1")?;
    
    let mut post_iter = stmt.query_map([post_id], |row| {
        Ok(Post {
            id: row.get(0)?,
            title: row.get(1)?,
            date: row.get(2)?,
            body: row.get(3)?,
            author_id: row.get(4)?,
            author: row.get(5)?,
        })
    })?;

    post_iter.next().transpose()
}

// Create a new post
pub fn create_post(conn: &Connection, title: &str, date:&str, body: &str, author_id: usize) -> Result<usize> {
    let sql = "
        INSERT INTO posts (title, date, body, author_id)
        VALUES (?1, ?2, ?3, ?4)
        ";
    conn.execute(sql, &[title, date, body, &author_id.to_string()])
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

