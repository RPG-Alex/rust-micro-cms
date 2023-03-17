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

fn main(){
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
	);
	let mut stmt = conn.prepare("SELECT * from notes");
	let mut rows = stmt.query(rusqlite::params![]);
	while let Some(row) = rows.next() {
		let go = Post {
			id : row.get(0),
			title: row.get(1),
			body: row.get(2),
			time_stamp: row.get(3),
		};
		println!("{:?}",go);
	}   
}


