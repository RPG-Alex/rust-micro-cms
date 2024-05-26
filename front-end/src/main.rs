mod app;
mod handlers;
mod routes;
mod models;
mod views;
mod errors;
mod api;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
