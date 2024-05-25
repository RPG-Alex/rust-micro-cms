use crate::models::{posts::NewPost};
use chrono::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NewPostProps {
    new_post: NewPost,
}

#[function_component(PostForm)]
pub fn post_form() -> Html {
    let mut new_post = NewPost::new();
    let is_preview = use_state(|| false);
    let is_draft = use_state(|| false);

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

    html!{
        <>
        if *is_preview {
            <div class="post">
                <h2>{ &new_post.title }</h2>
                <h3>{ format!("{}", Utc::now().date_naive()) }</h3>
                <p>{ &new_post.body }</p>
            </div>
            <div class="preview">
                <div class="form-buttons posts">
                    <button class="submit-button" onclick={on_update_click}>{ "Back to edit" }</button>
                </div>
            </div>
        } else {
            <div class="posts">
                <form>
                    <textarea id="post_form" />
                    <div class="form-buttons">
                        <button type="button" onclick={on_preview_click} class="preview-button">{ "Preview" }</button>
                        <input type="radio" checked={new_post.draft} id="draft-check" />
                        <label for="draft-check">{"draft"}</label>
                        <input type="submit" value="Submit New Post" class="submit-button" />
                        <input type="reset" value="reset" class="reset-button" />
                    </div>
                </form>
            </div>
        }
    </>
    }
}