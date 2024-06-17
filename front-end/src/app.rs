use crate::api::fetch_posts;
use crate::models::nav::Nav;
use crate::models::{posts::Posts, styling::Style};
use crate::routes::CMSRoutes;
use crate::views::{nav_bar::NavBar, styling::styling::StyleInjector};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let default_style = Style::default();

    let posts = use_state(Posts::default);
    let nav = use_state(Nav::default);
    let error_message = use_state(|| None);

    {
        let posts = posts.clone();
        let error_message = error_message.clone();
        use_effect_with((), move |_| {
            let posts = posts.clone();
            let error_message = error_message.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_posts().await {
                    Ok(fetched_posts) => {
                        posts.set(fetched_posts);
                    }
                    Err(e) => {
                        error_message.set(Some(e.to_string()));
                    }
                }
            });
            || ()
        });
    }
    
    html! {
        <div>
            <title>{ "Micro CMS!" }</title>

            <NavBar nav={(*nav).clone()}/>
            <BrowserRouter>
                <ContextProvider<Posts> context={(*posts).clone()}>
                    <CMSRoutes />
                </ContextProvider<Posts>>
            </BrowserRouter>
            <StyleInjector style={default_style} />
            {
                if let Some(error) = (*error_message).as_ref() {
                    html! { <div class="error-message">{ error }</div> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
