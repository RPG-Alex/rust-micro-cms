use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewAuthor {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Author {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub deleted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateAuthor {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub deleted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Authors {
    pub authors: Vec<Author>,
}
