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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub draft: bool,
}

impl NewPost {
    pub fn new() -> Self {
        NewPost {
            title: "".to_string(),
            body: "".to_string(),
            draft: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePost {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub archived: bool,
    pub draft: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Posts {
    pub posts: Vec<Post>,
}

impl Default for Posts {
    fn default() -> Self {
        Posts { posts: Vec::new() }
    }
}
