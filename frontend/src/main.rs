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

#[derive(Properties, PartialEq)]
struct PostsDetailsProps {
    post: Post,
}



#[function_component(PostDetails)]
fn post_details(PostsDetailsProps { post }: &PostsDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ post.title.clone() }</h3>
			<h4>{"Date: "}{post.date.clone()}</h4>
            <b>{ post.body.clone()}</b>
        </div>
    }
}

///Structure for a vector of blog posts
#[derive(Properties, PartialEq)]
struct PostsListProps {
	posts: Vec<Post>,
	on_click: Callback<Post>
}

///function for displaying list of blog posts
#[function_component(PostsList)]
fn posts_list(PostsListProps{posts, on_click }: &PostsListProps) -> Html{
	let on_click = on_click.clone();
	posts
		.iter()
		.map(|post| {
			let on_post_select = {
				let on_click = on_click.clone();
				let post = post.clone();
				Callback::from(move |_| {
					on_click.emit(post.clone())
				})
			};
			

			html!{
			<p>
				<a key={post.id} onclick={on_post_select}>{format!("{}", post.title)}</a>
			</p>
			}
		}).collect()

}

#[derive(Clone, PartialEq, Debug)]
enum AddPostState{
	Button, 
	Form,
	Submitted,
}

#[function_component(AddPost)]
fn add_post_form() -> Html {
	let state = use_state(|| AddPostState::Button);

	let switch_to_form = {
		let state = state.clone();
		Callback::from(move |_| {
			state.set(AddPostState::Form);
		})
	};

	

	match *state {
		AddPostState::Button => {
			html!{
				<button onclick={switch_to_form}>{"Add new Post"}</button>
			}
		}
		AddPostState::Form => {
			let submit_form = {
				let state = state.clone();
				Callback::from(move |_: yew::MouseEvent| {
					let post_data = "Post Data Here";
					state.set(AddPostState::Submitted);
				})
			};
			html!{
				<>
					<input type="text" />
					<button onclick={submit_form}>{"Submit"}</button>
				</>
			}
		}
		AddPostState::Submitted => {
			html!{
				<>
				<b> {"Post Submitted!"}</b>
				</>
			}
			
		}
	}
}

#[function_component]
fn App() -> Html {

	let selected_post = use_state(|| None);

	let on_post_select = {
		let selected_post = selected_post.clone();
		Callback::from(move |post: Post| {
			selected_post.set(Some(post))
		})
	};

	let post_details = selected_post.as_ref().map(|post| html!{
		<PostDetails post={post.clone()} />
	});
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
		<header><a href={"index.html"}>{"Home"}</a></header>
		<PostsList posts={posts} on_click={on_post_select.clone()} />
		<AddPost />
		<footer>{"Copyright: "}<b>{COPYRIGHT_YEAR}</b></footer>
		{for post_details}
		</>
	}
}


fn main() {
	yew::Renderer::<App>::new().render();    
}
