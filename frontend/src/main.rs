use yew::prelude::*;
use chrono::{Datelike};

#[function_component]
fn App() -> Html {
	let creation_date = 2023;
	let current_year = chrono::Local::now().year();
	let mut title = "Welcome to the home page!";
	html!{
		<>
		<h1>{title}</h1>
		<footer>{"Copyright: "}<b>{creation_date}{"-"}{current_year}</b></footer>
		</>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();    
}
