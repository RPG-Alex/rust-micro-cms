use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct Post {
    pub id: Option<i64>,
    pub title: String,
    pub date: String, // stored in db as string
    pub body: String,
    pub archived: Option<bool>,
    pub draft: Option<bool>,
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
            archived: None,
            draft: None,
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
