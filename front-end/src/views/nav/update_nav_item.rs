use crate::errors::*;
use crate::handlers::nav::handle_update_nav_item;
use crate::models::nav::{NavItem, NavItemType};
use crate::routes::Routes;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew::Callback;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UpdateNavItemProps {
    pub update_nav: NavItem,
}

#[function_component(UpdateNavItemForm)]
pub fn update_nav_item_form(props: &UpdateNavItemProps) -> Html {
    let post_id = use_state(|| props.update_nav.id.clone());
    let item_type = use_state(|| props.update_nav.item_type.clone());
    let content = use_state(|| props.update_nav.content.clone());
    let url = use_state(|| props.update_nav.url.clone());
    html! {
        <div>
            {"Boiler plate for updating nav message"}
        </div>
    }
}
