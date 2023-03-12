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
		} else {
			//grab the timestamp and convert to string to put into db
			let local: DateTime<Local> = Local::now();
			let local = &local.to_string();

			//need to update this when I decide databse logic.
			conn.execute("INSERT INTO notes (title,body,time_stamp) values (?1,?2,?3)",
			[trimmed_body,local,trimmed_body])?;
		}
	}
	Ok(())
}


/*
To do:
	implement timestamp when adding post
		likely use chrono for this as unix timestamp needs to be made into string to enter in DB
	very if unique type is necessary for body/title fields
	implement CRUD functionality (rough first, then as API)
		First just read out to console
		Then as API - requires configuring server detals
	Decide on login system. 
		Likely add separate db table for logins
		Needs to hash password and can update password etc
	Implement Server Functionality
		GET 
		POST
		DELETE
		UPDATE
	Connect a simple front end for testing in browser
	Decide on how to handle markup - WYSIWYG editor markup?
*/