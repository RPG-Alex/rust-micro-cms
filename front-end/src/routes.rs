use crate::models::{posts::Posts, styling::Style};
use crate::views::{
    posts::{
        all_posts::PostList, new_post::PostForm, recent_posts::RecentPosts, single_post::SinglePost,
    },
    styling::update_styling::StyleForm,
};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    Home,
    #[at("/all-posts")]
    AllPosts,
    #[at("/post/:id")]
    Post { id: i64 },
    #[at("/post/new")]
    NewPost,
    #[at("/style_update")]
    Style,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(CMSRoutes)]
pub fn cms_routes() -> Html {
    let route = use_route::<Routes>();
    let posts_context = use_context::<Posts>().expect("context not found");

    match route {
        Some(Routes::Home) => html! {
            <RecentPosts posts={posts_context.posts.clone()} />
        },
        Some(Routes::AllPosts) => html! {
            <PostList posts={posts_context.posts.clone()} />
        },
        Some(Routes::Post { id }) => {
            if let Some(post) = posts_context.posts.iter().find(|p| p.id == id) {
                html! { <SinglePost post={post.to_owned()} /> }
            } else {
                html! { <h1>{"Post does not exist"}</h1> }
            }
        }
        Some(Routes::Style) => html! {
            <StyleForm style={Style::default()} posts={posts_context.posts.clone()} />
        },
        Some(Routes::NewPost) => html! {
            <PostForm />
        },
        Some(Routes::NotFound) | None => html! { <h1>{"404 Not Found"}</h1> },
    }
}
