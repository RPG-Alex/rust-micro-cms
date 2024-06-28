use std::result;

use crate::api;
use crate::errors::FrontendError;
use crate::models::styling::{NewStyle, Style};
use yew::{callback, prelude::*};

pub async fn handle_create_style(
    new_style: NewStyle,
    callback: Callback<Result<Style, FrontendError>>,
) {
    let result = api::add_style(new_style).await;
    callback.emit(result);
}

pub async fn handle_update_style(
    updated_style: Style,
    callback: Callback<Result<Style, FrontendError>>,
) {
    let result = api::update_style(updated_style).await;
    callback.emit(result);
}

pub async fn handle_delete_style(style_id: i64, callback: Callback<Result<(), FrontendError>>) {
    let result = api::delete_style(style_id).await;
    callback.emit(result);
}
