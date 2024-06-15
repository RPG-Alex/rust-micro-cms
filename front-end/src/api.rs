use crate::errors::FrontendError;
use crate::models::nav::NewNavItem;
use crate::models::{posts::*, styling::Style, nav::*};
use gloo_net::http::Request;

pub const ROOT_URL: &str = "http://127.0.0.1:3000";

pub async fn fetch_posts() -> Result<Posts, FrontendError> {
    match Request::get(&format!("{}/posts", ROOT_URL)).send().await {
        Ok(response) => match response.json::<Posts>().await {
            Ok(fetched_posts) => Ok(fetched_posts),
            Err(_) => Err(FrontendError::FetchError),
        },
        Err(_) => Err(FrontendError::NetworkError(
            "Failed to connect to the server".to_string(),
        )),
    }
}

pub async fn create_post(new_post: NewPost) -> Result<Post, FrontendError> {
    match Request::post(&format!("{}/posts", ROOT_URL))
        .json(&new_post)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => match response.json::<Post>().await {
            Ok(created_post) => Ok(created_post),
            Err(_) => Err(FrontendError::FetchError),
        },
        Err(_) => Err(FrontendError::NetworkError(
            "Failed to connect to the server".to_string(),
        )),
    }
}

pub async fn update_post(updated_post: UpdatePost) -> Result<UpdatePost, FrontendError> {
    let id = updated_post.id;
    match Request::put(&format!("{}/posts/{}", ROOT_URL, id))
        .json(&updated_post)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => match response.json::<UpdatePost>().await {
            Ok(post) => Ok(post),
            Err(_) => Err(FrontendError::FetchError),
        },
        Err(_) => Err(FrontendError::NetworkError(
            "Failed to connect to the server".to_string(),
        )),
    }
}

pub async fn delete_post(id: i64) -> Result<(), FrontendError> {
    match Request::delete(&format!("{}/posts/{}", ROOT_URL, id))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(FrontendError::NetworkError(
            "Failed to connect to the server".to_string(),
        )),
    }
}

pub async fn add_style(style: Style) -> Result<Style, FrontendError> {
    match Request::post(&format!("{}/styles", ROOT_URL))
        .json(&style)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => match response.json::<Style>().await {
            Ok(created_style) => Ok(created_style),
            Err(_) => Err(FrontendError::FetchError),
        },
        Err(_) => Err(FrontendError::NetworkError(
            "Failed to connect to server".to_string(),
        )),
    }
}

pub async fn add_nav_item(new_item: NewNavItem) -> Result<NavItem, FrontendError> {
    match Request::post(&format!("{}/nav", ROOT_URL))
        .json(&new_item)
        .unwrap()
        .send()
        .await {
            Ok(response) => match response.json::<NavItem>().await {
                Ok(created_item) => Ok(created_item),
                Err(_) => Err(FrontendError::FetchError)
            },
            Err(_) => Err(FrontendError::NetworkError(
                "Failed to connect to server".to_string(),
            )),
        }
}
