use crate::handlers::{nav, posts, styling};
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
        // Nav Routes
        .route(
            "/nav",
            get(nav::fetch_nav_items_handler)
                .post(nav::create_nav_item_handler)
                .delete(nav::delete_nav_item_handler),
        )
}
