use crate::errors::*;
use crate::handlers::nav::handle_delete_nav_item;
use crate::models::nav::{NavItem, NavItemType, NewNavItem};
use crate::routes::Routes;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew::Callback;
use yew_router::prelude::*;

#[function_component(DeleteNavItem)]
