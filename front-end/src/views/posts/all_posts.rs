use yew::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::posts::Post;

#[function_component(PostList)]
pub fn post_list(props: &PostListProps) -> Html {
    html! {
        <div class="posts">
            { for props.posts.iter().map(|post| {
                html! {
                    <div class="post">
                        <h2>{ &post.title }</h2>
                        <h3>{ format!("by {} on {}", &post.author, &post.date) }</h3>
                        <p>{ &post.body }</p>
                    </div>
                }
            })}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PostListProps {
    pub posts: Vec<Post>,
}

