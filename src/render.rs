use axum::{
    response::Html,
    Json,
};
use serde::{Deserialize};
use serde_json::Value;
use std::convert::Infallible;

// Temporary structure to deserialize each post from JSON
#[derive(Deserialize)]
pub struct TempPost {
    pub id: usize,
    pub title: String,
    pub date: String,
    pub body: String,
    pub author_id: usize,
    pub author: String,
}

// Deserialize the json and structure as html and return it
pub async fn posts(posts: Result<Json<String>, Infallible>) -> Html<String> {
    match posts {
        Ok(Json(json_string)) => {
            // Parse the JSON string to a serde_json::Value
            let posts_data: Value = serde_json::from_str(&json_string).unwrap();

            // Check if posts_data is an array and iterate over it
            if posts_data.is_array() {
                let posts_array = posts_data.as_array().unwrap();
                let mut html_output = String::new();

                for post_value in posts_array {
                    // Deserialize each post
                    let post: TempPost = serde_json::from_value(post_value.clone()).unwrap();

                    // Create HTML string for each post
                    html_output.push_str(&format!(
                        "<div><h2>{}</h2><p><i>{}</i></p><p>{}</p></div>",
                        post.title, post.author, post.body
                    ));
                }

                Html(html_output)
            } else {
                Html("<div>Error: Posts data is not an array.</div>".to_string())
            }
        }
        Err(_) => Html("<div>Unable to parse JSON</div>".to_string()),
    }
}