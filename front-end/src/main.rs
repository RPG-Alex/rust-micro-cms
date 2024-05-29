mod api;
mod app;
mod errors;
mod handlers;
mod models;
mod routes;
mod views;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
