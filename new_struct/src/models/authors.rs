use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Author {
    pub id: Option<usize>,
    pub name: String,
}

impl Author {
    pub fn new(name: String) -> Self {
        Author { id: None, name }
    }

}
