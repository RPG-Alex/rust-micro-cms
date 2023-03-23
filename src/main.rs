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
		//the id is set to one but not used (this is a placeholder for the actual id you would get from a query. May need to make two structures to avoid this, a New_Post and Post or similar?)
		id: 2,
		title: "Hello!".to_string(),
		body: "World!".to_string(),
		time_stamp: Local::now().to_string(),
	};
	//practice inserting into db
	let mut stmt = conn.execute("INSERT INTO posts (title,body,time_stamp) values (?1,?2,?3)",[&test.title,&test.body,&test.time_stamp]);

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
		println!("id: {}, title: {}. body: {}. Time: {}", go.id, go.title, go.body, go.time_stamp);
	}

	//Working Update Functionality
	let new_title = "This was changed! It works!";
	let id = 1.to_string();
	let mut stmt = conn.execute("UPDATE posts SET title = (?1) WHERE id = (?2)",[new_title,&id])?;
	
	//adding delete functionality
	let mut stmt = conn.execute("DELETE FROM posts WHERE id = (?1)",[id])?;


	Ok(())
}