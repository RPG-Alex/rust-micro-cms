use crate::models::posts::Post;
use crate::routes::Routes;
use chrono::{Duration, Local, NaiveDateTime};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(RecentPosts)]
pub fn recent_posts(props: &RecentPostsProps) -> Html {
    let now = Local::now().date_naive();
    let three_months_ago = now - Duration::days(90);

    let recent_posts: Vec<&Post> = props
        .posts
        .iter()
        .filter(|post| {
            NaiveDateTime::parse_from_str(&post.date, "%Y-%m-%d %H:%M:%S")
                .map(|post_date| post_date.date() > three_months_ago)
                .unwrap_or(false)
        })
        .collect();

    html! {
        <div class="posts">
            { for recent_posts.iter().map(|post| {
                html! {
                    <Link<Routes> to={Routes::Post {id: post.id }}>
                        <div class="post">
                            <h2>{ &post.title }</h2>
                            <h3>{ format!("{}", &post.date) }</h3>
                            <p>{ &post.body }</p>
                        </div>
                    </Link<Routes>>
                }
            })}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct RecentPostsProps {
    pub posts: Vec<Post>,
}
