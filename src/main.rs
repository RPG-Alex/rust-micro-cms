use std::io;
use rusqlite::{Connection, Result};
use chrono::prelude::*;


// not sure about the structure setup for the main function - pretty sure this is debugging oriented. Will need to update. Delete comment when done.
fn main() -> Result<(), Box<dyn std::error::Error>> {
	
	let conn = Connection::open("post.db")?;
	conn.execute(
	
	//Should create the atabase if needed. 
		"create table if not exists notes (
			id integer primary key,
			title text not null unique,
			body text not null unique,
			time_stamp text not null unique
		)",
		[],
	)?;
	let mut running = true;
	while running == true {
		let mut buffer = String::new();
		io::stdin().read_line(&mut buffer)?;

		let trimmed_body = buffer.trim();

		if trimmed_body == "" {
			running = false;
		} else if {
			// this needs to work with JSON I think. We should figure this out first. And pass JSON to this as a standard method (api time) and then just use this logic for adding/updating posts. for now this is a place holder
		} else {
			//grab the timestamp and convert to string to put into db
			let local: DateTime<Local> = Local::now();
			let local = &local.to_string();
			
			//CREATE query for new post
			conn.execute("INSERT INTO notes (title,body,time_stamp) values (?1,?2,?3)",
			[trimmed_body,local,trimmed_body])?;
		}
	}
	Ok(())
}


/*
To do:
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