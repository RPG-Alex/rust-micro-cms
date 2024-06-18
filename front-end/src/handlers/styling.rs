use crate::api;
use crate::models::styling::{NewStyle,Style};
use crate::errors::FrontendError;
use yew::{callback, prelude::*};

pub async fn handle_create_style(
    new_style: NewStyle,
    callback: Callback<Result<Style, FrontendError>>,
) {
    let result = api::add_style(new_style).await;
    callback.emit(result);
}

//todo add method for updating style and deleting style. 