mod app;
mod handlers;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
