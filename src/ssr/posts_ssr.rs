use axum::{
    http::StatusCode,
    response::Html,
    Extension,
};
use handlebars::Handlebars;

// async fn add_post_form(Extension(handlebars): Extension<Handlebars<'_>>) ->  Result<Html<String>, StatusCode>{
//     let data = json!({});
//     let rendered = handlebars.render("add_post", &data)
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//     Ok(Html(render))
// }