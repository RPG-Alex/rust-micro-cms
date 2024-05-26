use crate::models::{posts::*, styling::Style};
use crate::errors::FrontendError;
use gloo_net::http::Request;

pub async fn fetch_posts() -> Result<Posts, FrontendError> {
    match Request::get("http://127.0.0.1:3000/posts")
        .send()
        .await
    {
        Ok(response) => match response.json::<Posts>().await {
            Ok(fetched_posts) => Ok(fetched_posts),
            Err(_) => Err(FrontendError::FetchError),
        },
        Err(_) => Err(FrontendError::NetworkError("Failed to connect to the server".to_string())),
    }
}

pub async fn create_post(new_post: NewPost) -> Result<Post, FrontendError> {
    match Request::post("http://127.0.0.1:3000/posts")
        .json(&new_post)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => match response.json::<Post>().await {
            Ok(created_post) => Ok(created_post),
            Err(_) => Err(FrontendError::FetchError),
        },
        Err(_) => Err(FrontendError::NetworkError("Failed to connect to the server".to_string())),
    }
}

pub async fn update_post(id: i64, updated_post: Post) -> Result<Post, FrontendError> {
    match Request::put(&format!("http://127.0.0.1:3000/posts/{}", id))
        .json(&updated_post)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => match response.json::<Post>().await {
            Ok(post) => Ok(post),
            Err(_) => Err(FrontendError::FetchError),
        },
        Err(_) => Err(FrontendError::NetworkError("Failed to connect to the server".to_string())),
    }
}

pub async fn delete_post(id: i64) -> Result<(), FrontendError> {
    match Request::delete(&format!("http://127.0.0.1:3000/posts/{}", id))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(FrontendError::NetworkError("Failed to connect to the server".to_string())),
    }
}