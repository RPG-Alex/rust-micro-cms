use crate::errors::*;
use crate::handlers::nav::handle_create_nav_item;
use crate::models::nav::{NewNavItem, NavItem, NavItemType, Nav};
use crate::routes::Routes;
use chrono::Utc;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::prelude::*;
use yew::Callback;

#[function_component(NewNavItemForm)]
pub fn new_nav_item_form() -> Html {
    let item_type = use_state(|| NavItemType::SocialLink); 
    let content = use_state(|| String::new());
    let url = use_state(|| String::new());
    let error_message = use_state(|| None);
    let navigator = use_navigator().unwrap();

    let on_item_type_change = {
        let item_type = item_type.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            item_type.set(NavItemType::from_str(&value));
        })
    };

    let on_content_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };

    let on_url_input = {
        let url = url.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            url.set(input.value());
        })
    };

    let onsubmit = {
        let item_type = item_type.clone();
        let content = content.clone();
        let url = url.clone();
        let error_message = error_message.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let new_nav_item = NewNavItem {
                item_type: *item_type,
                content: (*content).clone(),
                url: if url.is_empty() { None } else { Some((*url).clone()) },
            };
            let error_message = error_message.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                handle_create_nav_item(new_nav_item, Callback::from(move |result: Result<NavItem, FrontendError>| {
                    match result {
                        Ok(nav_item) => {
                            navigator.push(&Routes::Home);
                        }
                        Err(e) => {
                            error_message.set(Some(e.to_string()));
                        }
                    }
                })).await;
            });
        })
    };

    html! {
        <div class="nav-items">
            <form onsubmit={onsubmit}>
                <select onchange={on_item_type_change}>
                    <option value="ThumbnailUrl">{"Thumbnail URL"}</option>
                    <option value="BlogSummary">{"Blog Summary"}</option>
                    <option value="SocialLink">{"Social Link"}</option>
                </select>
                <textarea id="nav_item_form" value={(*content).clone()} oninput={on_content_input} />
                <input type="text" placeholder="URL (optional)" value={(*url).clone()} oninput={on_url_input} />
                <div class="form-buttons">
                    <input type="submit" value="Submit New Nav Item" class="submit-button" />
                    <input type="reset" value="reset" class="reset-button" />
                </div>
            </form>
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