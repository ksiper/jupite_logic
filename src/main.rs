#[macro_use]
extern crate diesel;
extern crate serde;
extern crate serde_derive;

use actix_web::{middleware, web, App, HttpRequest, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub mod config;
pub mod account;

use std::fs;
use config::Config;
use account::basic::*;


fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // prepare configurations from toml files
    let cfg_contents = fs::read_to_string("./conf/config.toml")
        .expect("Something went wrong reading the file");

    let cfg: Config = toml::from_str(cfg_contents.as_str()).unwrap();

    // init database connections pool
    println!("{}", cfg.basic.database_url);
    let dbmr = ConnectionManager::<MysqlConnection>::new(cfg.basic.database_url);
    let pool = r2d2::Pool::builder()
        .build(dbmr)
        .expect("failed to create pool");


    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .service(web::resource("/register").to(register))
    })
    .bind("0.0.0.0:8080")?
    .run()
}
