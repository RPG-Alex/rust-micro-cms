use yew::prelude::*;
use chrono::Datelike;

// Define the copyright year as a constant (year of copyright will never change)
const COPYRIGHT_YEAR: u16 = 2023;

///Structure for a blog post
#[derive(Clone,PartialEq)]
struct Post {
	id: usize,
	title: String,
	date: String,
	body: String,
}

///Structure for a vector of blog posts
#[derive(Properties, PartialEq)]
struct PostsListProps {
	posts: Vec<Post>,
}

///function for displaying list of blog posts
#[function_component(PostsList)]
fn posts_list(PostsListProps { posts }: &PostsListProps) -> Html{
	posts
		.iter()
		.map(|post| html!{
			<p>
				<a key={post.id} href={post.id.to_string()}>{format!("{}", post.title)}</a>
			</p>
		}).collect()
}

#[function_component]
fn App() -> Html {
	let home = "index.html";
	//test blog posts list. 
	let posts = vec![
		Post{
			id: 1,
			title: String::from("This is generated from a struct!"),
			date: chrono::Local::now().year().to_string(),
			body: String::from("Lorem ipsum dolor sit amet consectetur adipisicing elit. Eligendi, voluptas sed sapiente hic dolores qui, beatae eaque at repellendus illum aliquid animi laudantium labore dolorum unde fuga vero, sit perspiciatis."),
		},
		Post{
			id: 2,
			title: String::from("This is another generated from a struct!"),
			date: chrono::Local::now().year().to_string(),
			body: String::from("This post is only visible of you click! This post is only visible of you click! This post is only visible of you click! This post is only visible of you click! This post is only visible of you click! This post is only visible of you click! This post is only visible of you click! This post is only visible of you click!"),
		}
	];
	html!{
		<>
		<header><a href={home}>{"Home"}</a></header>
		<PostsList posts={posts} />
		<footer>{"Copyright: "}<b>{COPYRIGHT_YEAR}</b></footer>
		</>
	}
}


fn main() {
	yew::Renderer::<App>::new().render();    
}
