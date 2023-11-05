# Rust Micro CMS Backend

Welcome to this Rust made server backend!

## Technology

This project uses Rust and also the Axum framework!

## Under Construction

This backend is currently under construction.

## TODO - Roadmap

-  [ ]  **CREATE REST API for frontend to access**

    -  [ ]  **Database Setup**:
        -  [ ] Establish a database connection.
        -  [ ] Create tables for posts (and potentially users).
        -  [ ] Implement user authentication and authorization.
        -  [ ] Consider enabling HTTPS support (e.g., with a reverse proxy like Nginx or with Rust libraries like `hyper-tls`).
    
    -  [ ]  **Post Endpoints**:
        -  [ ] Create routes and handlers for the following APIs:
            -  [ ] Fetch all posts (HTTP GET /api/posts).
            -  [ ] Fetch a specific post by ID (HTTP GET /api/posts/{id}).
            -  [ ] Create a new post (HTTP POST /api/posts).
            -  [ ] Update a specific post (HTTP PUT /api/posts/{id}).
            -  [ ] Delete a specific post (HTTP DELETE /api/posts/{id}).
            -  [ ] Update post attributes (e.g., date, body) individually (HTTP PATCH /api/posts/{id}/date, HTTP PATCH /api/posts/{id}/body).
    
    -  [ ] [ ] **User Authentication**:
        -  [ ] Implement user registration and login APIs (HTTP POST /api/register, HTTP POST /api/login).
        -  [ ] Add middleware for user authentication and authorization to protect sensitive endpoints.
    
    -  [ ]  **Error Handling**:
        -  [ ] Implement consistent error handling and responses for API endpoints.
    
    -  [ ]  **API Documentation**:
        -  [ ] Generate and maintain API documentation, which can be served via a web page or a separate API documentation tool.
    
    -  [ ]  **Testing**:
        -  [ ] Write unit tests and integration tests for your API endpoints to ensure their correctness.
    
    -  [ ]  **Deployment**:
        -  [ ] Prepare your Axum application for deployment, considering factors like load balancing, containerization, and continuous integration/continuous deployment (CI/CD) pipelines.
    
    -  [ ]  **Security**:
        -  [ ] Implement security best practices, including input validation, rate limiting, and protection against common web vulnerabilities (e.g., cross-site scripting, SQL injection).
    
    -  [ ]  **Logging**:
        -  [ ] Set up logging to monitor your application's behavior and diagnose issues.
    
    -  [ ]  **Performance**:
        -  [ ] Optimize the performance of your API, considering factors like database query optimization, caching, and concurrency control.

    -  [ ]  **Continuous Maintenance**:
        -  [ ] Regularly update dependencies and apply security patches.
        -  [ ] Monitor server and application performance and apply necessary optimizations.

    -  [ ]  **API Versioning**:
        -  [ ] Consider implementing versioning for your APIs to maintain compatibility as the API evolves.

    -  [ ]  **Rate Limiting and Quotas**:
        -  [ ] Implement rate limiting and usage quotas to protect against abuse of your API.

    -  [ ]  **Data Validation**:
        -  [ ] Ensure that data received from the client is properly validated and sanitized to prevent security vulnerabilities.

    -  [ ]  **CORS and Cross-Origin Requests**:
        -  [ ] Configure CORS (Cross-Origin Resource Sharing) to control which domains can access your API.

    -  [ ]  **Automated Testing**:
        -  [ ] Set up automated testing for your API endpoints to ensure ongoing functionality and detect regressions.
