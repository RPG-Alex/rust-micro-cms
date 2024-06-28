use crate::errors::*;
use crate::handlers::posts::handle_create_post;
use crate::models::posts::{NewPost, Post};
use crate::routes::Routes;
use chrono::Utc;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew::Callback;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NewPostProps {
    pub new_post: NewPost,
}

#[function_component(PostForm)]
pub fn post_form() -> Html {
    let post_title = use_state(|| String::new());
    let post_body = use_state(|| String::new());
    let is_draft = use_state(|| true); // Default to draft
    let is_preview = use_state(|| false);
    let error_message = use_state(|| None);
    let navigator = use_navigator().unwrap();

    let on_preview_click = {
        let is_preview = is_preview.clone();
        Callback::from(move |_| {
            is_preview.set(true);
        })
    };

    let on_update_click = {
        let is_preview = is_preview.clone();
        Callback::from(move |_| {
            is_preview.set(false);
        })
    };

    let on_title_input = {
        let post_title = post_title.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            post_title.set(input.value());
        })
    };

    let on_body_input = {
        let post_body = post_body.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            post_body.set(input.value());
        })
    };

    let on_draft_change = {
        let is_draft = is_draft.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            is_draft.set(input.checked());
        })
    };

    let onsubmit = {
        let post_title = post_title.clone();
        let post_body = post_body.clone();
        let is_draft = is_draft.clone();
        let error_message = error_message.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let new_post = NewPost {
                title: (*post_title).clone(),
                body: (*post_body).clone(),
                draft: *is_draft,
            };
            let error_message = error_message.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                handle_create_post(
                    new_post,
                    Callback::from(move |result: Result<Post, FrontendError>| match result {
                        Ok(post) => {
                            let post_id: i64 = post.id;
                            navigator.push(&Routes::Post { id: post_id });
                        }
                        Err(e) => {
                            error_message.set(Some(e.to_string()));
                        }
                    }),
                )
                .await;
            });
        })
    };

    html! {
        <>
        if *is_preview {
            <div class="post">
                <h2>{ &*post_title }</h2>
                <h3>{ format!("{}", Utc::now().date_naive()) }</h3>
                <p>{ &*post_body }</p>
            </div>
            <div class="preview">
                <div class="form-buttons posts">
                    <button class="submit-button" onclick={on_update_click}>{ "Back to edit" }</button>
                </div>
            </div>
        } else {
            <div class="posts">
                <form onsubmit={onsubmit}>
                    <input type="text" placeholder="Title" value={(*post_title).clone()} oninput={on_title_input} />
                    <textarea id="post_form" value={(*post_body).clone()} oninput={on_body_input} />
                    <div class="form-buttons">
                        <button type="button" onclick={on_preview_click} class="preview-button">{ "Preview" }</button>
                        <input type="checkbox" checked={*is_draft} id="draft-check" onchange={on_draft_change} />
                        <label for="draft-check">{"Draft"}</label>
                        <input type="submit" value="Submit New Post" class="submit-button" />
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
        </>
    }
}
