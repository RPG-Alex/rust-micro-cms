use crate::handlers::{posts, styling};
use axum::{routing::get, Router};

pub async fn app_routes() -> Router {
    Router::new()
        //Post Routes
        .route(
            "/posts",
            get(posts::get_all_posts_handler).post(posts::create_post_handler),
        )
        .route(
            "/posts/:id",
            get(posts::get_post_by_id_handler)
                .put(posts::update_post_handler)
                .delete(posts::delete_post_handler),
        )
        // Styling Routes
        .route(
            "/styles",
            get(styling::fetch_styles_handler).post(styling::create_style_handler),
        )
}
