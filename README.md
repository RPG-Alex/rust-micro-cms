<a name="readme-top"></a>
# rust-micro-cms
<div align="center">
<img src="images/logo.png" alt="Logo" width="80" height="80">
</div>
<div align="center">
<img src="images/Rust-Micro-CMS-Structure.png" alt="Project Structure">
</div>
<details>
	<summary>Contents</summary>
	<ol>
		<li>
			<a href="#purpose">Purpose</a>
		</li>
		<li>
			<a href="#about-this-project">About this Project</a>
			<ul>
				<li><a href="#goals">Goals</a></li>
			</ul>
		</li>
		<li><a href="#contributing">Contributing</a></li>
	</ol>
</details>


## About this Project

Rust Micro CMS represents a streamlined approach to content management, designed with simplicity and ease of use at its core. Leveraging SQLite for file-based data storage, this system streamlines the processes of backup and migration, ensuring a hassle-free experience for users. Despite its minimalistic design, Rust Micro CMS doesn't compromise on functionality. It boasts a comprehensive suite of RESTful API features coupled with server-side rendering capabilities, catering to a broad spectrum of CMS requirements.

Currently under active development, Rust Micro CMS is continuously evolving, with new features being integrated to enhance its utility and performance. This project aims to provide a user-friendly, efficient, and adaptable CMS solution, minimizing configuration efforts while maximizing functionality.

### Goals

Originating as a project to deepen understanding of Rust, Rust Micro CMS has evolved with a focused mission: to provide a streamlined content management solution that leverages Rust's renowned performance and reliability. The current goal is to deliver a CMS with a minimalist footprint, ensuring swift, efficient operations while simplifying the complexities typically associated with content management systems. Rust Micro CMS is engineered to offer an uncomplicated yet robust platform, minimizing setup and operational overhead, and making it an ideal choice for projects where speed, security, and simplicity are paramount. Active development and enhancements aim to enrich its features, catering to the growing needs of its users while maintaining its core principle of minimalism.


## Contributing

If you want to contribute to this project, that is great! I am doing this to learn Rust and welcome anyone else doing the same or anyone that thinks a solution like this is something they would like to see happen!

### Setting Up Development Environment

To get started with Rust Micro CMS, you'll need to set up your development environment. This involves installing Rust, Cargo, and ensuring you have SQLite3 available for database operations. Follow the steps below to prepare your environment.

#### Prerequisites

- **Rust:** Rust Micro CMS is built with Rust, so you'll need to have Rust installed. If you haven't already, you can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install). This will also install Cargo, Rust's package manager and build system.
  
- **SQLite3:** This project uses SQLite for the database backend, storing data in a simple file-based database. Ensure SQLite3 is installed on your system. Most Unix-like operating systems come with SQLite pre-installed. For Windows, you may need to download and install SQLite manually from the [SQLite download page](https://www.sqlite.org/download.html).

#### Development Setup

1. **Clone the Repository:** Start by cloning the Rust Micro CMS repository to your local machine.
   ```bash
   git clone https://github.com/RPG-Alex/rust-micro-cms.git
   cd rust-micro-cms
   ```

2. **Install Dependencies:** While Rust Micro CMS primarily relies on Rust and Cargo-managed packages, ensure all dependencies are up to date by running:
   ```bash
   cargo update
   ```

3. **Environment Variables:** Copy the `.env.example` file to a new file named `.env` and update the `DATABASE_URL` to point to your SQLite database file. This file will be automatically created if it does not exist when you run the application.
   ```plaintext
   DATABASE_URL=sqlite:database.db
   ```

4. **Database Setup:** Rust Micro CMS will automatically handle database migrations and setup when you run the application. However, make sure SQLite3 is correctly installed and accessible from your command line.

  ##### SQL Configuration for Development
  - To have sqlx setup the databse for compile time checks please make sure you ahve sqlx installed:
  ```bash
  cargo install sqlx
  ```
  Then run: 
  ```bash
  sqlx database create
  ```
  ```bash
  sqlx migrate add <database name>
  ```

5. **Build and Run:** Compile and run the Rust Micro CMS with the following Cargo command:
   ```bash
   cargo run
   ```
   This command compiles the project and starts the server, making the CMS accessible locally.

6. **Accessing the CMS:** Once running, you can access the CMS through the API endpoints or by navigating to the provided web interface URL in your browser.



<p align="center">[<a href="#readme-top">RETURN TO TOP</a>]</p>
