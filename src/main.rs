//This file is just for testing things with the databasee structure. Transfer to new library when done
use rusqlite::{Connection, Result};
use chrono::prelude::*;

//for debugging so we can see what we are doing
#[derive(Debug)]
//desired structure we will use for posts
struct Post {
	id: i32,
	title: String,
	body: String,
	time_stamp: String,
}

fn main() -> Result<()> {
	let conn = Connection::open("notes.db");
	let mut note = String::new();
	conn.execute(
	"create table if not exists notes (
	id integer primary key,
	title text not null unique,
	body text not null unique,
	time_stamp text not null unique,
		)",
		[],
	)?;
	let mut stmt = conn.prepare("SELECT * from notes")?;
	let mut rows = stmt.query(rusqlite::params![])?;
	while let Some(row) = rows.next() {
		let go = Post {
			id : row.get(0),
			title: row.get(1),
			body: row.get(2),
			time_stamp: row.get(3),
		};
		println!("{:?}",go);
	}
	Ok(())
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