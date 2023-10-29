use leptos::*;
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
		<h1>"To do: show all posts!"</h1>
		<ul>
			{posts.into_iter()
				.map(|n| 
					view! {
					<li>
						{&n.title}
						<ShowSinglePostInfo post=n />
					</li>
				}).collect_view()
			}
		</ul>
    }
}

#[component]
fn ShowSinglePostInfo(post: Post) -> impl IntoView {
	
	view! {
		<h2>{post.date}</h2>
		<p>{post.body}</p>
	}
}