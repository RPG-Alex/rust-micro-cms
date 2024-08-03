use crate::errors::*;
use crate::handlers::nav::handle_delete_nav_item;
use crate::models::nav::{NavItem, NavItemType, NewNavItem};
use crate::routes::Routes;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew::Callback;
use yew_router::prelude::*;

#[derive(Properties,PartialEq)]
pub struct NavItemIdProps {
    pub id: i32,
}

#[function_component(DeleteNavItem)]
pub fn delete_nav_item(props: &NavItemIdProps) -> Html {

    html! {
        <div>
        {"Nav Item deleted"}
        </div>
    }
}