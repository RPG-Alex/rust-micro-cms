# Rust Micro CMS Backend

<div align="center">
  <img src="../images/logo.png" alt="Logo" width="80" height="80">
</div>

<div align="center">
  <img src="../images/Rust-Micro-CMS-Structure.png" alt="Project Structure">
</div>

<details>
  <summary>Contents</summary>
  <ol>
    <li><a href="#about-this-backend">About This Backend</a></li>
    <li><a href="#installation">Installation</a></li>
    <li><a href="#running">Running</a></li>
  </ol>
</details>

## About This Backend

Rust Micro CMS Backend is designed as a robust foundation for the Rust Micro CMS, facilitating efficient data management and providing a RESTful API interface. It uses `r2d2` with `rusqlite` to offer a performant, synchronous interface to a file-based database, making database operations seamless and swift. This backend caters to the architectural needs of modern web applications by offering scalability, ease of deployment, and integrated security measures.

## Installation

To set up the Rust Micro CMS Backend, follow these steps to prepare your environment:

### Prerequisites

- **Rust and Cargo:** Ensure you have Rust and Cargo installed. They are essential for building and managing dependencies.
  - Installation guide: [Official Rust Installation](https://www.rust-lang.org/tools/install).

- **SQLite3:** This project uses SQLite, a lightweight file-based database. Ensure SQLite3 is installed on your system.
  - For installation details, see [SQLite Download Page](https://www.sqlite.org/download.html).

### Environment Setup

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/your-username/rust-micro-cms-backend.git
   cd rust-micro-cms-backend
   ```
2. **Set Up Environment Variables**: Copy the .env.example to .env and modify it to suit your configuration.
   `DATABASE_URL=sqlite:./path/to/your/database.db`

3. **Database Initialization**: Initialize the database. The application will automatically handle database setup and migrations when you run it.

## Running

To run the backend server:

   ```bash
   cargo run
   ```
   This command starts the server which listens for API requests. You can interact with the backend using the defined RESTful endpoints through tools like cURL or Postman.

<p align="center">[<a href="#readme-top">RETURN TO TOP</a>]</p>