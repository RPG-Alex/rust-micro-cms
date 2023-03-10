# rust-micro-cms

## Purpose

This repository is an attempt to create a standalone content manage application (CMS) that utilizes the rust programming language for as much of the application as is possible. This includes CRUD with the DB (currently SQLite), and serving the application.

### Goals

Ultimately this project is a learning project, to help understand how Rust can be used (and possibly not used). But the goal for this project is create a rich contnet management system that someone could simply add the binaries to their desired server environment and then access and configure their installation, much like wordpress or other popular content management systems, through a browser client. 

The project is meant to have a small footprint, and hopefully offer a more reliable and secure CMS solution, with less overall logic and complexity.

## Intended Features:

* Full Database Accessbility - Out of the box the database should set itself up and operate without further configuration

* Model web server best practices, including properly configured CORS, and so on

* Compile to a single binary that can then be placed on your server and accessed via a browser

* Ease of backup - Using a file for DB with SQlite should aid in the ease of backup and avoid complications

* other features may surely arise



### Contributing

If you want to contribute to this project that is great! I am doing this to learn Rust and welcome anyone else doing the same or anyone that thinks a solution like this is something they would like to see happen!
