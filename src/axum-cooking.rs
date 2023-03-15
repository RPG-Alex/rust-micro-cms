//This document is for reference only - DELETE when its no longer needed
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}



	// let conn = Connection::open("post.db")?;
	// conn.execute(
	
	// //Should create the atabase if needed. 
	// 	"create table if not exists notes (
	// 		id integer primary key,
	// 		title text not null unique,
	// 		body text not null unique,
	// 		time_stamp text not null unique
	// 	)",
	// 	[],
	// )?;
	// let mut running = true;
	// while running == true {
	// 	//this CLI stuff needs to be replaced with Server logic later
	// 	let mut buffer = String::new();
	// 	io::stdin().read_line(&mut buffer)?;
	// 	let trimmed_body = buffer.trim();
	// 	//eventually this needs to take the HTTP header (POST, etc) then validate and do thing or return invalid		

	// 	if trimmed_body == "" {
	// 		println!("K. Bye");
	// 		//this terminates the loop
	// 		running = false;
	// 	} else if  trimmed_body == "/add" {
	// 		// this needs to work with JSON I think. We should figure this out first. And pass JSON to this as a standard method (api time) and then just use this logic for adding/updating posts. for now this is a place holder
	// 		println!("Whats the title?");
	// 		let mut title = String::new();
	// 		io::stdin().read_line(&mut title);
	// 		println!("Whats the post body content?");
	// 		let mut body = String::new();
	// 		io::stdin().read_line(&mut body);
	// 		//grab the timestamp and convert to string to put into db
	// 		let local: DateTime<Local> = Local::now();
	// 		let local = &local.to_string();
			
	// 		//CREATE query for new post
			
	// 		conn.execute("INSERT INTO notes (title,body,time_stamp) values (?1, ?2, ?3)",
	// 		[&title,&body,&local])?;
	// 		println!("Row added. Type /list to see all rows");
	// 	} else if trimmed_body == "/list" {
	// 		let mut stmt = conn.prepare("SELECT * from notes")?;
	// 		let mut rows = stmt.query(rusqlite::params![])?;
	// 		while let Some(row) = rows.next()? {
	// 			let id: i32 = row.get(0)?;
	// 			let title: String = row.get(1)?;
	// 			let body: String = row.get(2)?;
	// 			let time_stamp: String = row.get(3)?;
	// 			println!("id: {}, title: {}, body: {}, timestamp: {}",id,title.to_string(),body.to_string(),time_stamp.to_string());
    // 		}
	// 	}
	// 	else {
	// 		println!("You gotat say /add or nothing. If you say nothing I'm leaving though.");
	// 	}
	// }


    //sqlite logic:
	let conn = Connection::open("post.db")?;
	//creates db if not present
	conn.execute(
		"create table if not exists notes (
			id integer primary key,
			title text not null unique,
			body text not null unique,
			time_stamp text not null unique
		)",
		[],
	)?;

	let mut stmt = conn.prepare("SELECT * from notes")?;
	let mut rows = stmt.query(rusqlite::params![])?;
	let mut output = String::new();
	while let Some(row) = rows.next()? {
		let id: i32 = row.get(0)?;
		let title: String = row.get(1)?;
		let body: String = row.get(2)?;
		let time_stamp: String = row.get(3)?;
		output.push_str(&id.to_string());
		output.push_str(&title);
		output.push_str(&body);
		output.push_str(&time_stamp);
	}

	async fn all_posts(output) -> &'static str {
		&output
	}