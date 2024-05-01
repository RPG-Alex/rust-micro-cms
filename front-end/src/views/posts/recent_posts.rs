use yew::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::posts::Post;
use chrono::{NaiveDate, Local, Duration};

#[function_component(RecentPosts)]
pub fn recent_posts(props: &RecentPostsProps) -> Html {
    let now = Local::today().naive_local();
    let three_months_ago = now - Duration::days(90);

    let recent_posts: Vec<&Post> = props.posts.iter()
        .filter(|post| {
            NaiveDate::parse_from_str(&post.date, "%Y-%m-%d")
                .map(|post_date| post_date > three_months_ago)
                .unwrap_or(false)
        })
        .collect();

    html! {
        <div class="posts">
            { for recent_posts.iter().map(|post| {
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
pub struct RecentPostsProps {
    pub posts: Vec<Post>,
}
