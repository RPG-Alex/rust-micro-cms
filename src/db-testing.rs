//This file is just for testing things with the databasee structure. Transfer to new library when done

//for debugging so we can see what we are doing
#[derive(Debug)]
//desired structure we will use for posts
struct Posts {
	id: i32,
	title: String,
	body: String,
	time_stamp: String,
}

fn main(){
    	//just testing if the structure compiles
	let go = Posts {
		id : 1,
		title: String::from("Wazzup"),
		body: String::from("Lorem ipsum dolor sit amet consectetur adipisicing elit. Reiciendis sit nulla quia asperiores porro commodi aut impedit numquam molestiae quisquam iusto quae, voluptatibus eum sunt doloribus pariatur distinctio totam quibusdam."),
		time_stamp: String::from("today I guess"),
	};
    println!("{:?}",go);
}