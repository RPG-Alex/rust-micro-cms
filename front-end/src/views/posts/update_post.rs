use crate::handlers::posts::{handle_update_post, handle_delete_post};
use crate::errors::*;
use crate::models::posts::{Post, UpdatePost};
use crate::routes::Routes;
use chrono::Utc;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::prelude::*;
use yew::Callback;

#[derive(Properties, PartialEq)]
pub struct UpdatePostProps {
    pub post: Post,
}

#[function_component(UpdatePostForm)]
pub fn update_post_form(props: &UpdatePostProps) -> Html {
    let post_title = use_state(|| props.post.title.clone());
    let post_body = use_state(|| props.post.body.clone());
    let is_draft = use_state(|| props.post.draft);
    let is_archived = use_state(|| props.post.archived);
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

    let on_archived_change = {
        let is_archived = is_archived.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            is_archived.set(input.checked());
        })
    };

    let onsubmit = {
        let post_id = props.post.id;
        let post_title = post_title.clone();
        let post_body = post_body.clone();
        let is_draft = is_draft.clone();
        let is_archived = is_archived.clone();
        let error_message = error_message.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let updated_post = UpdatePost {
                id: post_id,
                title: (*post_title).clone(),
                body: (*post_body).clone(),
                archived: *is_archived,
                draft: *is_draft,
            };
            let error_message = error_message.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                handle_update_post(updated_post, Callback::from(move |result: Result<UpdatePost, FrontendError>| {
                    match result {
                        Ok(_) => {
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

    let on_delete_click = {
        let post_id = props.post.id;
        let error_message = error_message.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            if web_sys::window().unwrap().confirm_with_message("Are you sure you want to delete this post?").unwrap() {
                let error_message = error_message.clone();
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    handle_delete_post(post_id, Callback::from(move |result: Result<(), FrontendError>| {
                        match result {
                            Ok(_) => {
                                navigator.push(&Routes::Home);

                            }
                            Err(e) => {
                                error_message.set(Some(e.to_string()));
                            }
                        }
                    })).await;
                });
            }
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
                    <input type="hidden" value={props.post.id.to_string()} />
                    <input type="text" placeholder="Title" value={(*post_title).clone()} oninput={on_title_input} />
                    <textarea id="post_form" value={(*post_body).clone()} oninput={on_body_input} />
                    <div class="form-buttons">
                        <button type="button" onclick={on_preview_click} class="preview-button">{ "Preview" }</button>
                        <input type="checkbox" checked={*is_draft} id="draft-check" onchange={on_draft_change} />
                        <label for="draft-check">{"Draft"}</label>
                        <input type="checkbox" checked={*is_archived} id="archived-check" onchange={on_archived_change} />
                        <label for="archived-check">{"Archived"}</label>
                        <input type="submit" value="Update Post" class="submit-button" />
                        <input type="reset" value="reset" class="reset-button" />
                        <button type="button" onclick={on_delete_click} class="delete-button">{ "Delete Post" }</button>
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
