use yew::prelude::*;
use chrono::Datelike;

const COPYRIGHT_YEAR: u16 = 2023;


//Structure for a blog post
struct Post {
	id: usize,
	title: String,
	date: String,
	body: String,
}


#[function_component]
fn App() -> Html {
	let home = "index.html";
	//test blog posts list. 
	let example_posts = vec![
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
	//Iterator for posts and potential blog post template
	let example_posts = example_posts.iter().map(|post| html!{
		<p>
		<a key={post.id} href={post.id.to_string()}>{format!("{}", post.title)}</a>
		</p>
	}).collect::<Html>();
	html!{
		<>
		<header><a href={home}>{"Home"}</a></header>
		{example_posts}
		<footer>{"Copyright: "}<b>{COPYRIGHT_YEAR}</b></footer>
		</>
	}
}


fn main() {
	yew::Renderer::<App>::new().render();    
}
