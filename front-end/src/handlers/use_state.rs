use crate::handlers::posts::Post;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub post: Post,
}

#[function_component(PostComponent)]
pub fn post_component(props: &PostProps) -> Html {
    let expanded = use_state(|| false);

    let toggle_expanded = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!*expanded))
    };

    // Determine if the "Read more" button should be displayed
    let should_show_read_more = props.post.body.chars().count() > 250;

    let expanded_content = if *expanded {
        html! { <article>{ &props.post.body }</article> }
    } else {
        html! {
            <article>
                {
                    &props.post.body[..props.post.body.char_indices().nth(250).map_or(props.post.body.len(), |(i, _)| i)]
                }
            </article>
        }
    };

    html! {
        <section class="post">
            <h2>{ &props.post.title }</h2>
            <h3>{ format!("by: {}", &props.post.author) }</h3>
            <p>{ &props.post.date }</p>
            { expanded_content }
            {
                if should_show_read_more {
                    html! {
                        <button class="read-more" onclick={toggle_expanded}>
                            { if *expanded { "Read less" } else { "Read more" } }
                        </button>
                    }
                } else {
                    html! {}
                }
            }
        </section>
    }
}
