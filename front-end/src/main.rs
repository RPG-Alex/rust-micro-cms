mod app;

mod routes;
mod models;
mod views;
mod errors;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
