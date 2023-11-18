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


#[cfg(not(feature = "ssr"))]
pub async fn fetch_api<T>(path: &str) -> Option<T>
where
    T: Serializable,
{
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    // abort in-flight requests if, e.g., we've navigated away from this page
    leptos::on_cleanup(move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });

    let json = gloo_net::http::Request::get(path)
        .abort_signal(abort_signal.as_ref())
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;

    T::de(&json).ok()
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

