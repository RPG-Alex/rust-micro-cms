use leptos::{*};
use chrono::Datelike;

struct Post {
	id: usize,
	title: String,
	date: String,
	body: String,
}

#[component]
pub fn ListAllBlogPost() -> impl IntoView {
    let posts = vec![
		Post{
			id: 1,
			title: String::from("This is generated from a struct!"),
			date: chrono::Local::now().date_naive().to_string(),
			body: String::from("Lorem ipsum dolor sit amet consectetur adipisicing elit. Eligendi, voluptas sed sapiente hic dolores qui, beatae eaque at repellendus illum aliquid animi laudantium labore dolorum unde fuga vero, sit perspiciatis."),
		},
		Post{
			id: 2,
			title: String::from("This is another generated from a struct!"),
			date: chrono::Local::now().date_naive().to_string(),
			body: String::from("This post is only visible of you click! This post is only visible of you click! This post is only visible of you click! This post is only visible of you click! This post is only visible of you click! This post is only visible of you click! This post is only visible of you click! This post is only visible of you click!"),
		}
	];
	//currently view is prototype for learning.
    view!{
		<h1>"To do: List all posts!"</h1>
		<div id="posts">
		{posts.into_iter()
			.map(|n| 
				view! {<ShowSinglePostInfo post=n/>}).collect_view()
		}
		</div>
    }
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
				<p></p>
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
		{show_body_closure}
		</div>
	}
}

