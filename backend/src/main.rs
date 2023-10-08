/* TODO:
- CREATE REST API for frontend to access
    - Create database control structure
    - Add functions for reading from database first
    - Implement HTTP GET request

*/

//started working on the post structure, may add an entry for an author as well
struct Post {
    id: usize,
    title: String,
    date: String,
    body: String,
}

struct Posts {
    posts: Vec<Post>,
}

fn main() {
    println!("Hello, world!");
}
