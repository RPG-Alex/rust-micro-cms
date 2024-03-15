use axum::{
    http::StatusCode,
    response::Html,
    Extension,
};
use handlebars::Handlebars;

// async fn add_author_form(Extension(handlebars): Extension<Handlebars<'_>>) -> Result<Html<String>, StatusCode> {
//     //let data = json!({});
//     let rendered = handlebars.render("add_author", &data)
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//     Ok(Html(rendered))
// }

// async fn update_author_form(author_id: String, Extension(handlebars): Extension<Handlebars<'_>>) -> Result<Html<String>, StatusCode> {
//     //let author_data = json!({"id": author_id /*, Other author data*/});
//     let rendered = handlebars.render("update_author", &author_data)
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//     Ok(Html(rendered))
// }
