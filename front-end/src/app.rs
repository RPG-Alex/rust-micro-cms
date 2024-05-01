use crate::models::posts::{self as post_model, Post};
use yew::prelude::*;
use crate::views::{
    nav_bar::NavBar,
    posts::all_posts::PostList
};

#[function_component(App)]
pub fn app() -> Html {
    // Example posts data
    let example_posts = vec![
        Post {
            id: 1,
            title: "First Post".to_string(),
            date: "2023-04-01".to_string(),
            body: "This is the first post's body content.".to_string(),
            archived: false,
            draft: false,
            author_id: 1,
            author: "Author One".to_string(),
        },
        Post {
            id: 2,
            title: "Second Post".to_string(),
            date: "2023-04-02".to_string(),
            body: "This is the second post's body content.".to_string(),
            archived: false,
            draft: false,
            author_id: 2,
            author: "Author Two".to_string(),
        },
    ];

    html! {
        <>
            <title>{ "Micro CMS!" }</title>

            <NavBar />

            <h1>{ "Blog Posts" }</h1>
            <PostList posts={example_posts} />
        </>
    }
}
