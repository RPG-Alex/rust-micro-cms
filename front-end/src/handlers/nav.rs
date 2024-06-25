use crate::api;
use crate::models::nav::{NavItemType, NavItem, NewNavItem, Nav};
use crate::errors::FrontendError;
use yew::prelude::*;

pub async fn handle_create_nav_item(
    new_nav_item: NewNavItem,
    callback: Callback<Result<NavItem, FrontendError>>,
) {
    let result = api::add_nav_item(new_nav_item).await;
    callback.emit(result);
}

pub async fn handle_update_nav_item(
    nav_item: NavItem,
    callback:Callback<Result<NavItem, FrontendError>>,
) {
    let result = api::update_nav_item(nav_item).await;
    callback.emit(result);
}

pub async fn handle_delete_nav_item(
    id: i64,
    callback: Callback<Result<(), FrontendError>>,
) {
    let result = api::delete_nav_item(id).await;
    callback.emit(result);
}