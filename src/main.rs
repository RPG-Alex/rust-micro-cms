//import our DB logic
pub mod db_lib;
use db_lib::test;
//import the server library
pub mod server_lib;
use server_lib::serve;




fn main() {
	test();
	serve();
}