use crate::models::{posts::Post, styling::Style};
use crate::views::posts::recent_posts::RecentPosts;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StyleProps {
    pub style: Style,
    pub posts: Vec<Post>,
}

#[function_component(StyleForm)]
pub fn style_form(props: &StyleProps) -> Html {
    let is_preview = use_state(|| false);
    let style = use_state(|| props.style.clone());
    let posts = props.posts.clone();

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

    let on_style_change = {
        let style = style.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
            {
                let mut new_style = (*style).clone();
                new_style.css = target.value();
                style.set(new_style);
            }
        })
    };

    html! {
        <>
            if *is_preview {
                <div class="preview">
                    <RecentPosts posts={posts} />
                    <style>
                        { &style.css }
                    </style>
                    <div class="form-buttons posts">
                        <button class="submit-button" onclick={on_update_click}>{ "Back to edit" }</button>
                    </div>
                </div>
            } else {
                <div class="posts">
                    <form>
                        <textarea id="style_update_text" value={style.css.clone()} oninput={on_style_change} />
                        <div class="form-buttons">
                            <button type="button" onclick={on_preview_click} class="preview-button">{ "Preview" }</button>
                            <input type="submit" value="update style" class="submit-button" />
                            <input type="reset" value="reset" class="reset-button" />
                        </div>
                    </form>
                </div>
            }
        </>
    }
}
