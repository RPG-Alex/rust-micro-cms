use axum::{
	routing::{get,post},
	http::StatusCode,
	response::IntoResponse,
	Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;



//this should be our main function, to serve the whole thing
#[tokio::main]
pub async fn serve() -> () {
	let serve = Router::new()
		.route("/",get(root).post(received))
		//need to configure this -- review how axum is setting this up for clarity
		.route("/posts",get(posts));
	let addr = SocketAddr::from(([127,0,0,1],3000));
	axum::Server::bind(&addr)
		.serve(serve.into_make_service())
		.await
		.unwrap();
}

//functional hello world
async fn root() -> &'static str {
	"This is working at least"
}
async fn posts() -> &'static str {
    "And here will go posts"
}
async fn received() -> &'static str {
	"We get it. You sent us a post request"
}