use crate::errors::*;
use crate::handlers::nav::;
use crate::models::posts::{NewPost, Post};
use crate::routes::Routes;
use chrono::Utc;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::prelude::*;
use yew::Callback;

#[derive(Properties, PartialEq)]
pub struct NewPostProps {
    pub new_post: NewPost,
}