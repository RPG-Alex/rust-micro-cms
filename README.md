<a name="readme-top"></a>
# rust-micro-cms
<div align="center">
<img src="images/logo.png" alt="Logo" width="80" height="80">
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
		<li><a href="#roadmap">Roadmap</a></li>
		<li><a href="#contributing">Contributing</a></li>
	</ol>
</details>


## About this Project

Rust Micro CMS represents a streamlined approach to content management, designed with simplicity and ease of use at its core. Leveraging SQLite for file-based data storage, this system streamlines the processes of backup and migration, ensuring a hassle-free experience for users. Despite its minimalistic design, Rust Micro CMS doesn't compromise on functionality. It boasts a comprehensive suite of RESTful API features coupled with server-side rendering capabilities, catering to a broad spectrum of CMS requirements.

Currently under active development, Rust Micro CMS is continuously evolving, with new features being integrated to enhance its utility and performance. This project aims to provide a user-friendly, efficient, and adaptable CMS solution, minimizing configuration efforts while maximizing functionality.

### Goals

Originating as a project to deepen understanding of Rust, Rust Micro CMS has evolved with a focused mission: to provide a streamlined content management solution that leverages Rust's renowned performance and reliability. The current goal is to deliver a CMS with a minimalist footprint, ensuring swift, efficient operations while simplifying the complexities typically associated with content management systems. Rust Micro CMS is engineered to offer an uncomplicated yet robust platform, minimizing setup and operational overhead, and making it an ideal choice for projects where speed, security, and simplicity are paramount. Active development and enhancements aim to enrich its features, catering to the growing needs of its users while maintaining its core principle of minimalism.

## Roadmap:

### Backend Development
- [ ] **Server-side rendering with Rust**
    - [ ] Implement server-side rendering for HTML pages.
        - [ ] Server Side rendering functionality added
    - [ ] Serve HTML content directly from the server.
        - [ ] Create views for all views used for server side rendering

- [ ] **Backend built with Rust using Axum**
	- [ ] Design and implement a robust backend system to handle requests from the frontend.
	- [ ] Ensure the backend is secure and can handle high traffic.

- [ ] **Database Management System**
	- [x] Choose an appropriate DBMS for the project (considering factors like scalability, performance, etc.).
	- [x] Design the database schema and set up the database.
	- [x] Implement functionality for basic CRUD (Create, Read, Update, Delete) operations.

- [ ] **User Authentication**
	- [ ] Implement a secure login system for users.
	- [ ] Include features like password recovery, email verification, etc.
    - [ ] Implement user authentication and authorization.
    - [ ] Consider enabling HTTPS support (e.g., with a reverse proxy like Nginx or with Rust libraries like `hyper-tls`).
- [ ] **CREATE REST API for frontend to access**
    - [ ] **Database Setup**:
        - [x] Establish a database connection.
        - [x] Create tables for posts
        - [x] Create tables for users
    - [ ] **Post Endpoints**:
        - [ ] Create routes and handlers for the following APIs:
            - [ ] Fetch all posts (HTTP GET /api/posts).
                - [ ] Add pagination to fetch all posts.
            - [ ] Fetch a specific post by ID (HTTP GET /api/posts/{id}).
            - [ ] Create a new post (HTTP POST /api/posts).
            - [ ] Update a specific post (HTTP PUT /api/posts/{id}).
            - [ ] Delete a specific post (HTTP DELETE /api/posts/{id}).
            - [ ] Update post attributes (e.g., date, body) individually (HTTP PATCH /api/posts/{id}/date, HTTP PATCH /api/posts/{id}/body).
    - [ ] **User Authentication**:
        - [ ] Implement user registration and login APIs (HTTP POST /api/register, HTTP POST /api/login).
        - [ ] Add middleware for user authentication and authorization to protect sensitive endpoints.
    - [ ] **Error Handling**:
        - [ ] Implement consistent error handling and responses for API endpoints.
    - [ ] **API Documentation**:
        - [ ] Generate and maintain API documentation, which can be served via a web page or a separate API documentation tool.
    - [ ] **Testing**:
        - [ ] Write unit tests for:
            - [ ]  API endpoints
            - [ ]  Database queries
            - [ ]  Rendering
            - [ ]  POST Functionality
            - [ ]  GET Functionality
    - [ ] **Deployment**:
        - [ ] Prepare the Axum application for deployment, considering factors like load balancing, containerization, and continuous integration/continuous deployment (CI/CD) pipelines.
        - [ ] Consider dockerizing application.
    - [ ] **Security**:
        - [ ] Implement security best practices, including input validation, rate limiting, and protection against common web vulnerabilities (e.g., cross-site scripting, SQL injection).
    - [ ] **Logging**:
        - [ ] Set up logging to monitor the application's behavior and diagnose issues.
    - [ ] **Performance**:
        - [ ] Optimize the performance of the API, considering factors like database query optimization, caching, and concurrency control.
        - [ ] Undertake load testing to veriy load and optimize as needed. 
    - [ ] **Continuous Maintenance**:
        - [ ] Regularly update dependencies and apply security patches.
        - [ ] Monitor server and application performance and apply necessary optimizations.
    - [ ] **API Versioning**:
        - [ ] Consider implementing versioning for the APIs to maintain compatibility as the API evolves.
    - [ ] **Rate Limiting and Quotas**:
        - [ ] Implement rate limiting and usage quotas to protect against abuse of the API.
    - [ ] **Data Validation**:
        - [ ] Ensure data recieved is safe and valid

- [ ] **Separation of Concerns**
    - [ ] Separate db logic from controller logic
    - [ ] Implement a modular codebase for ease of extensibility and refactoring

### Additional Features

- [ ] **SEO Optimization**
	- [ ] Implement SEO best practices to improve the visibility of the CMS on search engines.

- [ ] **Accessibility**
	- [ ] Ensure that the CMS is accessible to all users, including those with disabilities.

### Documentation & Community

- [ ] **Documentation**
	- [ ] Write comprehensive documentation covering all aspects of the CMS, including setup, usage, troubleshooting, etc.

- [ ] **Community**
	- [ ] Encourage community involvement through contributions, feedback, etc.

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
   git clone https://yourrepositorylink/rust-micro-cms.git
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

5. **Build and Run:** Compile and run the Rust Micro CMS with the following Cargo command:
   ```bash
   cargo run
   ```
   This command compiles the project and starts the server, making the CMS accessible locally.

6. **Accessing the CMS:** Once running, you can access the CMS through the API endpoints or by navigating to the provided web interface URL in your browser.



<p align="center">[<a href="#readme-top">RETURN TO TOP</a>]</p>
