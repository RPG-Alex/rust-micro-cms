use crate::errors::*;
use crate::handlers::nav::handle_update_nav_item;
use crate::models::nav::{NavItem, NavItemType};
use crate::routes::Routes;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew::Callback;
use yew_router::prelude::*;



#[function_component(UpdateNavItemForm)]
pub fn update_nav_item_form(props: &UpdateNavItemProps) -> Html {

}