use crate::models::{posts::Posts, styling::Style};
use crate::routes::CMSRoutes;
use crate::views::{nav_bar::NavBar, styling::styling::StyleInjector};
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

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

            <NavBar />
            <BrowserRouter>
                <ContextProvider<Posts> context={(*posts).clone()}>
                    <CMSRoutes />
                </ContextProvider<Posts>>
            </BrowserRouter>
            <StyleInjector style={default_style} />
        </div>
    }
}
