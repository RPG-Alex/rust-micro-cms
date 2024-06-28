use crate::api;
use crate::errors::FrontendError;
use crate::models::posts::{NewPost, Post, UpdatePost};
use yew::prelude::*;

pub async fn handle_create_post(
    new_post: NewPost,
    callback: Callback<Result<Post, FrontendError>>,
) {
    let result = api::create_post(new_post).await;
    callback.emit(result);
}

pub async fn handle_update_post(
    updated_post: UpdatePost,
    callback: Callback<Result<UpdatePost, FrontendError>>,
) {
    let result = api::update_post(updated_post).await;
    callback.emit(result);
}

pub async fn handle_delete_post(id: i64, callback: Callback<Result<(), FrontendError>>) {
    let result = api::delete_post(id).await;
    callback.emit(result);
}
