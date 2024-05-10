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
enum Routes {
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
    
    let posts = use_state(Posts::default);
    {
        let posts = posts.clone();
        use_effect_with((), move |_| {
            let posts = posts.clone();
            wasm_bindgen_futures::spawn_local(async move {
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


    html! {
        <div>
            <title>{ "Micro CMS!" }</title>
            <StyleInjector style={default_style} />
            <NavBar />
            <BrowserRouter>
                <Switch<Routes> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn switch(routes: Routes) -> Html {
    let posts = 
    match routes {
        Routes::Home => html! {
            <RecentPosts posts={posts.posts.clone()} />
        },
        Routes::AllPosts => html! {
            <PostList posts={posts.posts.clone()} />
        },
        Routes::Post { id } => {
            if let Some(post) = posts.posts.iter().find(|p| p.id == id) {
                html! { <SinglePost post={post.to_owned()} /> }
            } else {
                html! { <h1>{"Post does not exist"}</h1> }
            }
        },
        Routes::Style => html! {
            <StyleForm style={Style::default()} />
        },
        Routes::NotFound => html! { <h1>{"404 Not Found"}</h1> },
    }
}