#![allow(dead_code)]
#![allow(unstable_name_collisions)]

/* Declare the mods */
mod conf;
mod database;
mod router;
mod serve;
mod util;

/// Use of configuration
use conf::CONF;

/// Use of actix web
use actix_web::{App, HttpServer};
/// use of opentelemetry
use opentelemetry::global;
/// use of tracing
use tracing_appender::rolling;
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::registry()
        // register hourly_logger
        .with({
            let hourly_logger = fmt::layer()
                .pretty()
                .with_target(true)
                .with_line_number(true)
                .with_ansi(false)
                .with_writer(rolling::hourly("log", "tracelog_hourly.log"));
            hourly_logger
        })
        // register wholely_logger
        .with({
            let wholely_logger = fmt::layer()
                .pretty()
                .with_target(true)
                .with_line_number(true)
                .with_ansi(false)
                .with_writer(rolling::never("log", "tracelog_whole.log"));
            wholely_logger
        })
        // register console logger
        .with(fmt::layer().pretty().with_writer(std::io::stdout))
        // register open telemetry jaeger tracer
        .with({
            global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
            let jaeger_tracer = tracing_opentelemetry::layer().with_tracer(
                opentelemetry_jaeger::new_agent_pipeline()
                    .with_service_name("fleaxj")
                    .install_simple()
                    .unwrap(),
            );
            jaeger_tracer
        })
        // initialize the tracing subscriber
        .init();

    HttpServer::new(|| App::new().configure(router::router))
        .bind(CONF.bind_args())
        .unwrap()
        .run()
        .await
}
