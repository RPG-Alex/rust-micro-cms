use axum::{
    http::StatusCode,
    Json,
    response::Html,
    Extension,
};
use handlebars::Handlebars;

pub async fn add_post_form() ->  Result<Html<String>, StatusCode>{
    //todo: wriet me
    Ok(Html(rendered))
}