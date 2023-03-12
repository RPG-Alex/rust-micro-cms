use std::io;
use rusqlite::{Connection, Result};
use std::time::SystemTime;

//Currently this code is based on this tutorial: https://levelup.gitconnected.com/how-to-build-a-note-taking-command-line-application-with-rust-part-1-34b9cd5be6b9
fn main() -> Result<(), Box<dyn std::error::Error>> {

	let now = SystemTime::now();
	println!("{:?} is the current time",now);
	
	// let conn = Connection::open("post.db")?;
	// conn.execute(
	// //Eventually need to add a title and date:  title text not null unique,
	// 	"create table if not exists notes (
	// 		id integer primary key,
	// 		body text not null unique
	// 	)",
	// 	[],
	// )?;
	// let mut running = true;
	// while running == true {
	// 	let mut buffer = String::new();
	// 	io::stdin().read_line(&mut buffer)?;
	// 	let trimmed_body = buffer.trim();
	// 	if trimmed_body == "" {
	// 		running = false;
	// 	} else {
	// 		//need to update this when I decide databse logic.
	// 		conn.execute("INSERT INTO notes (body) values (?1)",
	// 		[trimmed_body])?;
	// 	}
	// }
	Ok(())
}


// To do:
// Need to be able to get timestamp for posts, check there is title and body, and then input that into the database
// fn main() -> Result<(), Box<dyn std::error::Error>> {
    
//   	let conn = Connection::open("notes.db")?;
//   	conn.execute(
//     "create table if not exists notes (
//       id integer primary key,
//       body text not null unique
//     )",
//     [],
//   )?;  
//   let mut note = String::new();
//   let mut running = true;
//   while running {
	
//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer)?;
    
//     let trimmed_body = buffer.trim();
// 	let cmd_split = trimmed_body.split_once(" ");

//     let mut cmd = trimmed_body;
//     let mut msg = "";
// 	if cmd_split != None {
// 		cmd = cmd_split.unwrap().0;
// 		msg = cmd_split.unwrap().1;
// 	}

//     if trimmed_body == "/exit" {
//     	if !note.is_empty() {
//     		conn.execute("INSERT INTO notes (body) values (?1)", [&note])?;
//     	}
//      	running = false;
//     } else if trimmed_body == "/add"{
//     	conn.execute("INSERT INTO notes (body) values (?1)", [&note])?;
//     	note.clear();
//     } else if trimmed_body == "/list" {
//     	let mut stmt = conn.prepare("SELECT id, body from notes")?;
//     	let mut rows = stmt.query(rusqlite::params![])?;
//     	while let Some(row) = rows.next()? {
//     		let id: i32 = row.get(0)?;
//     		let body: String = row.get(1)?;
//     		println!("id: {} \ntext: {}", id, body.to_string());
//     	}
//     } else if cmd == "/del"{
//   		let id = msg;
// 		conn.execute("DELETE FROM NOTES WHERE id = (?1)", [id])?;
//     } else if cmd == "/edit" {
//     	let msg_split = msg.split_once(" ").unwrap();
//     	let id = msg_split.0;
//     	let body = msg_split.1;

//     	conn.execute("UPDATE notes SET body = (?1) WHERE id = (?2)",[body,id])?;
//     } else {
//     	//This needs to be fixed. Need to read up on str vs String again, getting confused here. 
//     	let mut add_to_body = trimmed_body.to_string();
//     	add_to_body.push_str(" \n");
//     	//
//     	note.push_str(add_to_body);
//     }
//   }  Ok(())
// }
