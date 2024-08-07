use crate::models::{
    nav::{Nav, NavItem, NavItemType},
    posts::Posts,
    styling::Style,
};
use crate::views::{
    nav::{new_nav_item::NewNavItemForm, update_nav_item::UpdateNavItemForm},
    posts::{
        all_posts::PostList, new_post::PostForm, recent_posts::RecentPosts,
        single_post::SinglePost, update_post::UpdatePostForm,
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
    #[at("/post/:id/edit")]
    UpdatePost { id: i64 },
    #[at("/post/new")]
    NewPost,
    #[at("/style_update")]
    Style,
    #[at("/nav/new")]
    NewNav,
    #[at("/nav/:id/edit")]
    EditNav { id: i64 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(CMSRoutes)]
pub fn cms_routes() -> Html {
    let route = use_route::<Routes>();
    let posts_context = use_context::<Posts>().expect("posts context not found");
    let nav_items_context = use_context::<Nav>().expect("nav items context not found");

    match route {
        Some(Routes::Home) => {
            if posts_context.posts.is_empty() {
                html! { <h1 class="posts">{"Loading..."}</h1> }
            } else {
                html! { <RecentPosts posts={posts_context.posts.clone()} /> }
            }
        }
        Some(Routes::AllPosts) => {
            if posts_context.posts.is_empty() {
                html! { <h1 class="posts">{"Loading..."}</h1> }
            } else {
                html! { <PostList posts={posts_context.posts.clone()} /> }
            }
        }
        Some(Routes::Post { id }) => {
            if let Some(post) = posts_context.posts.iter().find(|p| p.id == id) {
                html! { <SinglePost post={post.to_owned()} /> }
            } else {
                html! { <h1 class="posts">{"Post does not exist"}</h1> }
            }
        }
        Some(Routes::UpdatePost { id }) => {
            if let Some(post) = posts_context.posts.iter().find(|p| p.id == id) {
                html! { <UpdatePostForm post={post.to_owned()} /> }
            } else {
                html! { <h1 class="posts">{"Post does not exist"}</h1> }
            }
        }
        Some(Routes::Style) => html! {
            <StyleForm style={Style::default()} posts={posts_context.posts.clone()} />
        },
        Some(Routes::NewPost) => html! {
            <PostForm />
        },
        Some(Routes::NewNav) => html! {
            <NewNavItemForm />
        },
        Some(Routes::EditNav { id }) => {
            if let Some(nav_item) = nav_items_context.items.iter().find(|p| p.id == id ) {
                html! {<UpdateNavItemForm nav_item={nav_item.to_owned()} />}
            } else {
                html! { <h1 class="posts">{"Nav Item Does Not Exist"}</h1>}
            }
            //placeholder value change later!
            // <UpdateNavItemForm update_nav={NavItem {
            //     id: 1,
            //     item_type: NavItemType::ThumbnailUrl,
            //     content: "Some content".to_string(),
            //     url: Some("website.url".to_string()),
            // }}/>
        },
        Some(Routes::NotFound) | None => html! { <h1 class="posts">{"404 Not Found"}</h1> },
    }
}
