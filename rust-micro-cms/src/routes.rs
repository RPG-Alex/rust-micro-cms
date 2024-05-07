use crate::handlers::{authors, posts};
use axum::{
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Router,
};

pub async fn app_routes() -> Router {
    Router::new()
        // Author routes
        .route(
            "/authors",
            get(authors::get_all_authors_handler).post(authors::create_author_handler),
        )
        .route(
            "/authors/:id",
            get(authors::get_author_by_id_handler).put(authors::update_author_handler),
        )
        // Post routes
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
}
