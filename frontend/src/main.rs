
use leptos::*;
// Define the copyright year as a constant (year of copyright will never change)
const COPYRIGHT_YEAR: u16 = 2023;

pub fn main() {
	mount_to_body(|| {
		view! {
			<hr />
			<footer><b>{"Copyright: "}{COPYRIGHT_YEAR}</b></footer>
		}
	})
}


