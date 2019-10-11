#[macro_use]
extern crate diesel;
extern crate serde;
extern crate serde_derive;

use actix_web::{middleware, web, App, HttpRequest, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub mod config;
pub mod account;

use config::Config;
use account::basic::*;


fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // prepare configurations
    let toml_str = r#"
        database_url = "mysql://weei:123456@localhost:3306/jpt_logic"
    "#;

    let decoded: Config = toml::from_str(toml_str).unwrap();
    println!("{}", decoded.database_url);

    let dbmr = ConnectionManager::<MysqlConnection>::new(decoded.database_url);
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
    .bind("127.0.0.1:8080")?
    .run()
}
