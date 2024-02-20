use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct Post {
    pub id: Option<i64>,
    pub title: String,
    pub date: String, //need to be a string for db querying
    pub body: String,
    pub author_id: i64, 
}

#[derive(Debug, Clone)]
pub struct Posts {
    pub posts: Vec<Post>,
}

impl Post {
    pub fn new(title: String, date: String, body: String, author_id: i64) -> Self {
        Post {
            id: None,
            title,
            date,
            body,
            author_id,
        }
    }
}

// May not be necessary
impl Posts {
    pub fn new(posts: Vec<Post>) -> Self {
        Posts { posts }
    }
}
