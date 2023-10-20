mod db;
use axum::{
    response::Html,
    routing::get,
    Router,
};
//Used for getting the socket address with Axum
use std::net::SocketAddr;
use std::path::Path;



#[tokio::main]
async fn main() {
    //Currently taken from the Axum Example
    let app = Router::new().route("/", get(handler));

    // Database Testing logic
    let db_path = Path::new("posts.db");
    let db_conn = db::establish_connection(&db_path);
    let db_create = db::create_posts_table(&db_conn.unwrap());
    let db_conn = db::establish_connection(&db_path);
    let db_insert = db::insert_post(&db_conn.unwrap(), "New Post Title", "2023-10-20", "This is the post body")
    .expect("Failed to insert post");

    // server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Axum Example handler
async fn handler() -> Html<&'static str> {
    Html("<h1>Work in Progress</h1>")
}

/* TODO:
- CREATE REST API for frontend to access

    1. Database Setup:
    - Establish a database connection.
    - Create tables for posts (and potentially users).
    - Implement user authentication and authorization .
    - Consider enabling HTTPS support (e.g., with a reverse proxy like Nginx or with Rust libraries like `hyper-tls`).
    
    2. Post Endpoints:
    - Create routes and handlers for the following APIs:
        - Fetch all posts (HTTP GET /api/posts).
        - Fetch a specific post by ID (HTTP GET /api/posts/{id}).
        - Create a new post (HTTP POST /api/posts).
        - Update a specific post (HTTP PUT /api/posts/{id}).
        - Delete a specific post (HTTP DELETE /api/posts/{id}).
        - Update post attributes (e.g., date, body) individually (HTTP PATCH /api/posts/{id}/date, HTTP PATCH /api/posts/{id}/body).
    
    3. User Authentication:
    - Implement user registration and login APIs (HTTP POST /api/register, HTTP POST /api/login).
    - Add middleware for user authentication and authorization to protect sensitive endpoints.
    
    4. Error Handling:
    - Implement consistent error handling and responses for API endpoints.
    
    5. API Documentation:
    - Generate and maintain API documentation, which can be served via a web page or a separate API documentation tool.
    
    6. Testing:
    - Write unit tests and integration tests for your API endpoints to ensure their correctness.
    
    7. Deployment:
    - Prepare your Axum application for deployment, considering factors like load balancing, containerization, and continuous integration/continuous deployment (CI/CD) pipelines.
    
    8. Security:
    - Implement security best practices, including input validation, rate limiting, and protection against common web vulnerabilities (e.g., cross-site scripting, SQL injection).
    
    9. Logging:
    - Set up logging to monitor your application's behavior and diagnose issues.
    
    10. Performance:
    - Optimize the performance of your API, considering factors like database query optimization, caching, and concurrency control.

    11. Continuous Maintenance:
    - Regularly update dependencies and apply security patches.
    - Monitor server and application performance and apply necessary optimizations.

    12. API Versioning:
    - Consider implementing versioning for your APIs to maintain compatibility as the API evolves.

    13. Rate Limiting and Quotas:
    - Implement rate limiting and usage quotas to protect against abuse of your API.

    14. Data Validation:
    - Ensure that data received from the client is properly validated and sanitized to prevent security vulnerabilities.

    15. CORS and Cross-Origin Requests:
    - Configure CORS (Cross-Origin Resource Sharing) to control which domains can access your API.

    16. Automated Testing:
    - Set up automated testing for your API endpoints to ensure ongoing functionality and detect regressions.
*/