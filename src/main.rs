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
use fleaxj::util::transform::Transformation;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 初始化Tracing
    tracing_config_initalize();

    // 初始化ActixWeb
    HttpServer::new(|| App::new().configure(router::router))
        // HTTP初始化
        .transformation(|server| server.bind(CONF.bind_args()).unwrap())
        // HTTPS初始化
        .transformation(|server| {
            if let Some((addrs, builder)) = CONF.use_ssl() {
                server.bind_openssl(addrs, builder.unwrap()).unwrap()
            } else {
                log::info!("You do not provide ssl/tls certs!");
                server
            }
        })
        // 启动程序
        .run()
        .await
}
