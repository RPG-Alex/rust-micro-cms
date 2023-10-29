use leptos::{*, html::h2};
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
	let (visibility, set_visibility) = create_signal(true);
    view!{
		<h1>"To do: show all posts!"</h1>
		<div id="posts">
		{posts.into_iter()
			.map(|n| 
				view! {
					<div id={&n.id.to_string()}
						on:click=move |_| {
							if visibility() == true {
								set_visibility.update(|b| *b = false);
							} else {
								set_visibility.update(|b| *b = true);
							}
						}
						>
						<h2>{&n.title}</h2>
						<ShowSinglePostInfo post=n visibility=visibility.get()/>
					</div>
			}).collect_view()
		}
		</div>
    }
}

#[component]
fn ShowSinglePostInfo(post: Post, visibility: bool) -> impl IntoView {
	view! {
		<i>{post.date}</i>
		<p hidden=visibility>{post.body}</p>
	}
}