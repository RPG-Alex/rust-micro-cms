use crate::models::posts::{self as post_model, Post};
use yew::prelude::*;
use crate::views::{
    nav_bar::NavBar,
    posts::{
        all_posts::PostList,
        recent_posts::RecentPosts,
    }
};

#[function_component(App)]
pub fn app() -> Html {
    // Example posts data
    let example_posts = vec![
        Post {
            id: 1,
            title: "First Post".to_string(),
            date: "2024-04-01".to_string(),
            body: "This is the first post's body content.".to_string(),
            archived: false,
            draft: false,
            author_id: 1,
            author: "Author One".to_string(),
        },
        Post {
            id: 2,
            title: "Second Post".to_string(),
            date: "2024-04-02".to_string(),
            body: "This is the second post's body content.".to_string(),
            archived: false,
            draft: false,
            author_id: 2,
            author: "Author Two".to_string(),
        },
        Post {
            id: 3,
            title: "Third Post".to_string(),
            date: "2023-04-03".to_string(),
            body: "Details about the third post.".to_string(),
            archived: false,
            draft: false,
            author_id: 3,
            author: "Author Three".to_string(),
        },
        Post {
            id: 4,
            title: "Fourth Post".to_string(),
            date: "2023-04-04".to_string(),
            body: "Insights from the fourth post.".to_string(),
            archived: false,
            draft: false,
            author_id: 4,
            author: "Author Four".to_string(),
        },
        Post {
            id: 5,
            title: "Fifth Post".to_string(),
            date: "2023-04-05".to_string(),
            body: "Discussion on the fifth post topic.".to_string(),
            archived: false,
            draft: false,
            author_id: 5,
            author: "Author Five".to_string(),
        },
        Post {
            id: 6,
            title: "Sixth Post".to_string(),
            date: "2023-04-06".to_string(),
            body: "Exploration of the sixth post's theme.".to_string(),
            archived: false,
            draft: false,
            author_id: 6,
            author: "Author Six".to_string(),
        },
        Post {
            id: 7,
            title: "Seventh Post".to_string(),
            date: "2023-04-07".to_string(),
            body: "Seventh post’s revelations and thoughts.".to_string(),
            archived: false,
            draft: false,
            author_id: 7,
            author: "Author Seven".to_string(),
        },
        Post {
            id: 8,
            title: "Eighth Post".to_string(),
            date: "2023-04-08".to_string(),
            body: "Analyzing the topic discussed in the eighth post.".to_string(),
            archived: false,
            draft: false,
            author_id: 8,
            author: "Author Eight".to_string(),
        },
        Post {
            id: 9,
            title: "Ninth Post".to_string(),
            date: "2023-04-09".to_string(),
            body: "Perspectives from the ninth post.".to_string(),
            archived: false,
            draft: false,
            author_id: 9,
            author: "Author Nine".to_string(),
        },
        Post {
            id: 10,
            title: "Tenth Post".to_string(),
            date: "2023-04-10".to_string(),
            body: "Summary of thoughts on the tenth topic.".to_string(),
            archived: false,
            draft: false,
            author_id: 10,
            author: "Author Ten".to_string(),
        },
    ];
    

    html! {
        <>
            <title>{ "Micro CMS!" }</title>

            <NavBar />

            <h1>{ "Blog Posts" }</h1>
            //<PostList posts={example_posts} />
            <RecentPosts posts={example_posts} />
        </>
    }
}
