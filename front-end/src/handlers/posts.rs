use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub date: String,
    pub body: String,
    pub archived: bool,
    pub draft: bool,
    pub author_id: i64,
    pub author: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub date: String,
    pub body: String,
    pub draft: bool,
    pub author_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePost {
    pub id: i64,
    pub title: String,
    pub date: String,
    pub body: String,
    pub archived: bool,
    pub draft: bool,
    pub author_id: i64,
}
