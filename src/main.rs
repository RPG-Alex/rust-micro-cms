use rusqlite::{Connection, Result};
use chrono::prelude::*;

//Post structure - Mirror Database Structure
struct Post {
	id: i32,
	title: String,
	body: String,
	time_stamp: String,
}

fn main() -> Result<()> {
	//setup connection to database and create it if does not exist
	let conn = Connection::open("posts.db")?;
	conn.execute(
	"create table if not exists posts (
	id integer primary key,
	title text not null unique,
	body text not null unique,
	time_stamp text not null unique
		)",
		[],
	)?;

	//Test post for checking structure and acccessign it.
	let test = Post {
		id: 1,
		title: "Welcome!".to_string(),
		body: "Hello World!".to_string(),
		time_stamp: Local::now().to_string(),
	};
	println!("id: {}, title: {}. body: {}. Time: {}",test.id,test.title,test.body,test.time_stamp);

	//This is fetching all entries and setting a variable to each one.
	let mut stmt = conn.prepare("SELECT * from posts")?;
	let mut rows = stmt.query(rusqlite::params![])?;
	while let Some(row) = rows.next()? {
		let go = Post {
			id : row.get(0)?,
			title: row.get(1)?,
			body: row.get(2)?,
			time_stamp: row.get(3)?,
		};
	}
	Ok(())
}