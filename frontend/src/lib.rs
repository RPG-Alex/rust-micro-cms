use leptos::{component, view, IntoView};
use leptos_router::*;
use leptos_meta::*;


mod routes;

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	view!{
		<>
			<Router>
				<Nav />
				<main>
					<Routes>
						
					</Routes>
				</main>
			</Router>
		</>
	}
}