use axum::*;


#[macro_use]
extern crate simple_log;

use simple_log::LogConfigBuilder;
use std::error::Error;

#[tokio::main]
async fn main() {
    
    let config = LogConfigBuilder::builder()
        .path("./log/rust_micro_cms.log")
        .size(1 * 1024 * 1024) // 1 MB
        .roll_count(5)
        .level("info")
        .output_file()
        .output_console()
        .build();

    simple_log::new(config);

    info!("Rust Micro CMS started");
}
