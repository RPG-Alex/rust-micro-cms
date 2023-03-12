use std::io;
use rusqlite::{Connection, Result};
use std::time::SystemTime;
use chrono::{NaiveDate, NaiveDateTime};


// not sure about the structure setup for the main function - pretty sure this is debugging oriented. Will need to update. Delete comment when done.
fn main() -> Result<(), Box<dyn std::error::Error>> {

	let now = SystemTime::now();
	println!("{:?} is the current time",now);
	
	let conn = Connection::open("post.db")?;
	conn.execute(
	//Not working, under construction:
	//Currently the database structure is sufficient for posts
		"create table if not exists notes (
			id integer primary key,
			title text not null unique,
			body text not null unique,
			time_stamp integer not null unique
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
			//need to update this when I decide databse logic.
			conn.execute("INSERT INTO notes (body,time_stamp) values (?1,?2)",
			[trimmed_body,])?;
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