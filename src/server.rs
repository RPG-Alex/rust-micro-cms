use axum::{
	routing::{get,post},
	http::StatusCode,
	response::IntoResponse,
	Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> () {


	let serve = Router::new()
		.route("/",get(root))
		//need to configure this -- review how axum is setting this up for clarity
		.route("/posts",get(root));
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