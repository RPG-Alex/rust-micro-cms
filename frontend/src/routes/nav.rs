use leptos::{component, view, IntoView};
use leptos_router::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <header class="header">
            <nav class="inner">
                <A href="/">
                    "Home"
                </A>
            </nav>
        </header>
    }
}