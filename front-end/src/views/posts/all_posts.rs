use crate::models::posts::Post;
use crate::routes::Routes;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(PostList)]
pub fn post_list(props: &PostListProps) -> Html {
    html! {
        <div class="posts">
            { for props.posts.iter().map(|post| {
                html! {
                    <Link<Routes> to={Routes::Post { id: post.id }}>
                        <div class="post">
                            <h2>{ &post.title }</h2>
                            <h3>{ format!("{}", post.date) }</h3>
                            <p>{ &post.body }</p>
                        </div>
                    </Link<Routes>>
                }
            })}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PostListProps {
    pub posts: Vec<Post>,
}
