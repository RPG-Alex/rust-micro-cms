use crate::models::{
    posts::{
        NewPost,
        Post,
        Posts,
        UpdatePost,
    }, 
    styling::Style
};
use crate::views::{
    nav_bar::NavBar,
    posts::{all_posts::PostList, recent_posts::RecentPosts, single_post::SinglePost},
    styling::{styling::StyleInjector, update_styling::StyleForm},
};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/all")]
    AllPosts,
    #[at("/post/:id")]
    Post { id: i64 },
    #[at("/style_update")]
    Style,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    let default_style = Style::default();
    // Change the state to hold Posts instead of Vec<Post>
    let posts = use_state(Posts::default);

    {
        let posts = posts.clone();
        use_effect_with((), move |_| {
            let posts = posts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // Fetch the Posts object and directly set the posts state
                let fetched_posts: Posts = Request::get("http://127.0.0.1:3000/posts")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                posts.set(fetched_posts);
            });
            || ()
        });
    }

    

    let route = use_route::<Route>().unwrap_or(Route::NotFound);

    let rendered_component = match route {
        Route::Home => html! {
            <RecentPosts posts={posts.posts.clone()} />
        },
        Route::AllPosts => html! {
            <PostList posts={posts.posts.clone()} />
        },
        Route::Post { id } => {
            if let Some(post) = posts.posts.iter().find(|p| p.id == id) {
                html! { <SinglePost post={post.to_owned()} /> }
            } else {
                html! { <h1>{"Post does not exist"}</h1> }
            }
        },
        Route::Style => html! {
            <StyleForm style={Style::default()} />
        },
        Route::NotFound => html! { <h1>{"404 Not Found"}</h1> },
    };

    html! {
        <div>
            <title>{ "Micro CMS!" }</title>
            <StyleInjector style={default_style} />
            <NavBar />
            <BrowserRouter>
                <div class="content">
                    { rendered_component }
                </div>
            </BrowserRouter>
        </div>
    }
}


// fn switch(routes: Route) -> Html {
//     // Example posts data
//     let example_posts = vec![
//         Post {
//             id: 1,
//             title: "First Post".to_string(),
//             date: "2024-04-01".to_string(),
//             body: "This is the first post's body content.".to_string(),
//             archived: false,
//             draft: false,
//         },
//         Post {
//             id: 2,
//             title: "Second Post".to_string(),
//             date: "2024-04-02".to_string(),
//             body: "This is the second post's body content.".to_string(),
//             archived: false,
//             draft: false,

//         },
//         Post {
//             id: 3,
//             title: "Third Post".to_string(),
//             date: "2023-04-03".to_string(),
//             body: "Details about the third post.".to_string(),
//             archived: false,
//             draft: false,

//         },
//         Post {
//             id: 4,
//             title: "Fourth Post".to_string(),
//             date: "2023-04-04".to_string(),
//             body: "Insights from the fourth post.".to_string(),
//             archived: false,
//             draft: false,

//         },
//         Post {
//             id: 5,
//             title: "Fifth Post".to_string(),
//             date: "2023-04-05".to_string(),
//             body: "Discussion on the fifth post topic.".to_string(),
//             archived: false,
//             draft: false,
//         },
//         Post {
//             id: 6,
//             title: "Sixth Post".to_string(),
//             date: "2023-04-06".to_string(),
//             body: "Exploration of the sixth post's theme.".to_string(),
//             archived: false,
//             draft: false,

//         },
//         Post {
//             id: 7,
//             title: "Seventh Post".to_string(),
//             date: "2023-04-07".to_string(),
//             body: "Seventh post’s revelations and thoughts.".to_string(),
//             archived: false,
//             draft: false,

//         },
//         Post {
//             id: 8,
//             title: "Eighth Post".to_string(),
//             date: "2023-04-08".to_string(),
//             body: "Analyzing the topic discussed in the eighth post.".to_string(),
//             archived: false,
//             draft: false,

//         },
//         Post {
//             id: 9,
//             title: "Ninth Post".to_string(),
//             date: "2023-04-09".to_string(),
//             body: "Perspectives from the ninth post.".to_string(),
//             archived: false,
//             draft: false,

//         },
//         Post {
//             id: 10,
//             title: "Tenth Post".to_string(),
//             date: "2023-04-10".to_string(),
//             body: "Summary of thoughts on the tenth topic.".to_string(),
//             archived: false,
//             draft: false,

//         },
//     ];



//     match routes {
//         Route::Home => html!(
//             <>
//                 <h1> {"Recent Posts"}</h1>
//                 <RecentPosts posts={example_posts} />
//             </>
//         ),
//         Route::AllPosts => html! {
//             <>
//                 <h1>{ "Blog Posts" }</h1>
//                 <PostList posts={example_posts} />
//             </>
//         },
//         Route::Post { id } => html! {

//             if let Some(post) = find_post(&example_posts, id ) {
//                 <SinglePost post={

//                     post.to_owned()
//                 } />
//             } else {
//                 <h1>{"Post does not exist"}</h1>
//             }

//         },
//         Route::Style => html!(
//             <>
//                 <h1>{"Update Blog Styling"}</h1>
//                 <StyleForm style={Style::default()} />
//             </>
//         ),
//         Route::NotFound => html! {
//             <h1>{"404 Not Found"}</h1>
//         },
//     }
// }

// fn find_post(posts: &[Post], id: i64) -> Option<&Post> {
//     posts.iter().find(|p| p.id == id)
// }
