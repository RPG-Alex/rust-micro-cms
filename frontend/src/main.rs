use yew::prelude::*;
use chrono::Datelike;

const COPYRIGHT_YEAR: u16 = 2023;


//Structure for a blog post
struct Post {
	title: String,
	date: String,
	body: String,
}


#[function_component]
fn App() -> Html {
	let home = "index.html";
	//test blog post. 
	let example_post = Post{
		title: String::from("This is generated from a struct!"),
		date: chrono::Local::now().year().to_string(),
		body: String::from("Lorem ipsum dolor sit amet consectetur adipisicing elit. Eligendi, voluptas sed sapiente hic dolores qui, beatae eaque at repellendus illum aliquid animi laudantium labore dolorum unde fuga vero, sit perspiciatis."),
	};
	html!{
		<>
		<header><a href={home}>{"Home"}</a></header>
		<h1>{example_post.title}</h1>
		<p>{example_post.body}</p>
		<footer>{"Copyright: "}<b>{COPYRIGHT_YEAR}{"-"}{example_post.date}</b></footer>
		</>
	}
}


fn main() {
	yew::Renderer::<App>::new().render();    
}
