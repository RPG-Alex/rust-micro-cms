use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav>
            <div class="content">
                <div class="img-container">
                     <h1 class="blog-title">
                        <a href="placeholder_url"> {"Place holder text"}</a>
                     </h1>
                </div>
                <p class="description">
                {"Placeholder of Blog Description"}
                </p>
                <ul class="links">
                    <li><a href="https://google.com"> {"Placeholder link"}</a></li>
                </ul>
            </div>
        </nav>
    }
}
