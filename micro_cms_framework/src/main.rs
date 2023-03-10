use std::io;
use rusqlite::{Connection, Result};


//Currently this code is based on this tutorial: https://levelup.gitconnected.com/how-to-build-a-note-taking-command-line-application-with-rust-part-1-34b9cd5be6b9
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let conn = Connection::open("post.db")?;
	conn.execute(
	//Eventually need to add a title and date:  title text not null unique,
		"create table if not exists notes (
			id integer primary key,
			body text not null unique
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
			conn.execute("INSERT INTO notes (body) values (?1)",
			[trimmed_body])?;
		}
	}
	Ok(())
}
