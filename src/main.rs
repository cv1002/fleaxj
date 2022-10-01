#![allow(dead_code)]
#![allow(unstable_name_collisions)]

/* Declare the mods */
mod conf;
mod database;
mod router;
mod serve;
mod util;

/// Use of actix web
use actix_web::{App, HttpServer};

/// Use of configuration
use conf::{tracing_config_initalize, CONF};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 初始化Tracing
    tracing_config_initalize();

    // 初始化ActixWeb
    HttpServer::new(|| App::new().configure(router::router))
        .bind(CONF.bind_args())
        .unwrap()
        .run()
        .await
}
