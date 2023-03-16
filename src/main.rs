use axum::{
	routing::{get,post},
	http::StatusCode,
	response::IntoResponse,
	Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use rusqlite::{Connection, Result};
use chrono::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


	let serve = Router::new()
		.route("/",get(root))
		//need to configure this -- review how axum is setting this up for clarity
		.route("/posts",get(root));
	let addr = SocketAddr::from(([127,0,0,1],3000));
	axum::Server::bind(&addr)
		.serve(serve.into_make_service())
		.await
		.unwrap();

	Ok(())
}


//functional hello world
async fn root() -> &'static str {
	"This is working at least"
}


/*
To do:
	Start working on this project like its a server side applicaton!
	Implement REST API server
		Configure Server 
		Establish functionalities
			GET 
			POST
			DELETE
			UPDATE
		Map CRUD to GPPU (
			Create = POST
			Read = GET
			UPDATE = PUT
			Delete = DELETE
		)
	Decide on login system. 
		Likely add separate db table for logins
		Needs to hash password and can update password etc
	Connect a simple front end for testing in browser
	Decide on how to handle markup - WYSIWYG editor markup?
*/