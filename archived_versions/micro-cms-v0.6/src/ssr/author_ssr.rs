use axum::{
    http::StatusCode,
    Json,
    response::Html,
    Extension,
};
use handlebars::Handlebars;

async fn add_author_form() -> Result<Html<String>, StatusCode> {
    //todo: write me
    
    Ok(Html(rendered))
}

async fn update_author_form() -> Result<Html<String>, StatusCode> {
    // todo: render me
    Ok(Html(rendered))
}
