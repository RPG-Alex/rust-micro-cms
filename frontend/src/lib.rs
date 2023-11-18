use leptos::{*, Serializable};
use serde::{Deserialize, Serialize};
//use chrono::Datelike;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Post {
	id: usize,
	title: String,
	date: String,
	body: String,
}



#[component]
pub fn ListAllBlogPost() -> impl IntoView {

}

#[component]
fn ShowSinglePostInfo(post: Post) -> impl IntoView {
	let (show_body, set_show_body) = create_signal(false);

	let post_body = post.body.clone();
	let show_body_closure = move || {
		if show_body.get() {
			view! {
				<p>{post_body.clone()}</p>
			}
		} else {
			view! {
				<p>{"Click to read this post!"}</p>
			}
		}
	};

	view! {
		<div on:click=move |_| {
			if show_body.get() {
				set_show_body(false)
			} else {
				set_show_body(true)
			}
		}>
		<h2>{post.title}</h2>
		<i>{post.date}</i>
		{show_body_closure()}
		</div>
	}
}

