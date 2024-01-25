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

This project's purpose is to create an easily deployable website CMS, with a complete backend, ultimately with a "What you see is what you get" (WYSIWYG) or similar input for ease of posting.

### Goals

Ultimately, this project is a learning project, to help understand how Rust can be used (and possibly not used). The goal for this project is to create a rich content management system that someone could simply add the binaries to their desired server environment and then access and configure their installation, much like WordPress or other popular content management systems, through a browser client.

The project is meant to have a small footprint and hopefully offer a more reliable and secure CMS solution, with less overall logic and complexity.

## Roadmap:

### Backend Development

- [ ] **Server-side rendering with Rust**
    - [ ] Implement server-side rendering for HTML pages.
        - [x] Server Side rendering functionality added
    - [ ] Serve HTML content directly from the server.
        - [ ] Create views for all views used for server side rendering

- [ ] **Backend built with Rust using Axum**
	- [ ] Design and implement a robust backend system to handle requests from the frontend.
	- [ ] Ensure the backend is secure and can handle high traffic.

- [ ] **Database Management System**
	- [x] Choose an appropriate DBMS for the project (considering factors like scalability, performance, etc.).
	- [ ] Design the database schema and set up the database.
	- [ ] Implement functionality for basic CRUD (Create, Read, Update, Delete) operations.

- [ ] **User Authentication**
	- [ ] Implement a secure login system for users.
	- [ ] Include features like password recovery, email verification, etc.
	- [ ] **CREATE REST API for frontend to access**
        - [ ] **Database Setup**:
            - [ ] Establish a database connection.
            - [ ] Create tables for posts
            - [ ] Create tables for users
            - [ ] Implement user authentication and authorization.
            - [ ] Consider enabling HTTPS support (e.g., with a reverse proxy like Nginx or with Rust libraries like `hyper-tls`).
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
                - [x]  API endpoints
                - [x]  Database queries
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

<p align="center">[<a href="#readme-top">RETURN TO TOP</a>]</p>
