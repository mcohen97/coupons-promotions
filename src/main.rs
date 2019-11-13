#![allow(dead_code)]
// usful in dev mode
use std::io;
use crate::server::{Server, ServerConfig};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate mime;
extern crate chrono;
extern crate iso3166_1;
extern crate iata_types;
extern crate http;
extern crate dotenv;
extern crate evalexpr;
extern crate env_logger;
extern crate lapin;
extern crate lapin_futures;
mod server;
mod services;
mod models;
mod schema;
mod messages;


fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    std::env::set_var("RUST_LOG", "info");
    let domain = std::env::var("HOST").unwrap_or("127.0.0.1".into());
    let port = std::env::var("PORT").unwrap_or("8080".into()).parse().expect("Invalid port");
    let db_host = std::env::var("DB_HOST").expect("DB_HOST missing");
    let db_user = std::env::var("DB_USER").expect("DB_USER missing");
    let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD missing");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME missing");
    let rabbit_url = std::env::var("RABBIT_URL").expect("RABBIT_URL missing");
    let logger_format = std::env::var("LOGGER_FORMAT").expect("LOGGER_FORMAT missing");

    info!("Server is staring at {}:{}", &domain, &port);
    let config = ServerConfig { domain, port, db_host, db_name, db_user, db_password, rabbit_url, logger_format };
    let server = Server::new(config);
    server.start()?;

    Ok(())
}

