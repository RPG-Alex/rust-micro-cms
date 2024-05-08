use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub date: String,
    pub body: String,
    pub archived: bool,
    pub draft: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub date: String,
    pub body: String,
    pub draft: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePost {
    pub id: i64,
    pub title: String,
    pub date: String,
    pub body: String,
    pub archived: bool,
    pub draft: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Posts {
    pub posts: Vec<Post>,
}
