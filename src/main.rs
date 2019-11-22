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
extern crate derive_more;
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
extern crate nanoid;
extern crate openssl_probe;
extern crate openssl;
extern crate jsonwebtoken;
#[macro_use]
extern crate lazy_static;

mod server;
mod services;
mod models;
mod schema;
mod messages;


fn main() -> io::Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv::dotenv().ok();
    env_logger::init();
    // std::env::set_var("RUST_LOG", "info,error,debug");
    debug!("DEBUG FAMA LAMA");
    let domain = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into()).parse().expect("Invalid port");
    let rabbit_url = std::env::var("RABBIT_URL").expect("RABBIT_URL missing");
    let logger_format = std::env::var("LOGGER_FORMAT").expect("LOGGER_FORMAT missing");

    info!("Server is staring at {}:{}", &domain, &port);
    let config = ServerConfig { domain, port, rabbit_url, logger_format };
    let server = Server::new(config);
    server.start()?;

    Ok(())
}
