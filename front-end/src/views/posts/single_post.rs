use crate::models::posts::Post;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct PostProps {
    pub post: Post,
}

#[function_component(SinglePost)]
pub fn post_view(props: &PostProps) -> Html {
    html! {
        <div class="posts">
            <div class="post">
                <h2>{ &props.post.title }</h2>
                <h3>{ format!("{}", &props.post.date) }</h3>
                <p>{ &props.post.body }</p>
            </div>
        </div>
    }
}
