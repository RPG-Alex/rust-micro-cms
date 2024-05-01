use serde::{Deserialize, Serialize};
/*
TODO:
- configure frontend to take json from backend
- json contains css settings
- use this struct to style web view
*/



//incomplete list of values for css
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Style {
    background_color: String,
    color: String,
    font_size: String,
}


// maybe have a default styler for css in case api fails
impl Style {
    pub fn new() -> Self {
        Self {
            background_color: "#ffffff".to_string(),
            color: "#000000".to_string(),
            font_size: "16px".to_string(),
        }
    }
}
