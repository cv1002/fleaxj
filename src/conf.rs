/// Use of opentelemetry
use opentelemetry::global;
/// Use of serde
use serde::{Deserialize, Serialize};
/// Use of tracing
use tracing_appender::rolling;
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt};

/// Use of other module
use crate::{util::lazy::Lazy, LazyNew};

#[derive(Serialize, Deserialize)]
pub struct DBArgs {
    pub user: String,
    pub host: String,
    pub database: String,
    pub password: String,

    /// To prevent from construct the DBArgs struct outside the module.
    private: Option<()>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfFile {
    port: u16,
    host: String,
    db: DBArgs,
}
pub static CONF: Lazy<ConfFile> =
    LazyNew!(toml::from_slice(&std::fs::read("./conf.toml").unwrap()).unwrap());
impl ConfFile {
    pub fn bind_args(&self) -> (&str, u16) {
        (&self.host, self.port)
    }
    pub fn database_args(&self) -> &DBArgs {
        &self.db
    }
}

pub fn tracing_config_initalize() {
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
}
