/// Use of openssl
use openssl::{error::ErrorStack, ssl::SslAcceptorBuilder};
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
pub struct SSLArgs {
    pub key: String,
    pub cert: String,
    pub port: u16,
    pub host: String,

    /// To prevent from construct the DBArgs struct outside the module.
    #[serde(skip)]
    private: (),
}
impl SSLArgs {
    pub fn ssl_bind_args(&self) -> (&str, u16) {
        (self.host.as_str(), self.port)
    }
    pub fn ssl_builder(&self) -> Result<SslAcceptorBuilder, ErrorStack> {
        use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_private_key_file(&self.key, SslFiletype::PEM)?;
        builder.set_certificate_chain_file(&self.cert)?;
        Ok(builder)
    }
}

/// Configuration of database
#[derive(Serialize, Deserialize)]
pub struct DBArgs {
    pub user: String,
    pub host: String,
    pub database: String,
    pub password: String,

    /// To prevent from construct the DBArgs struct outside the module.
    #[serde(skip)]
    private: (),
}

/// Configuration file format
/// # Example
///
/// ```toml
/// port = 8080
/// host = "localhost"
/// [db]
/// user = "****"
/// host = "****:3306"
/// database = "****"
/// password = "****"
/// [ssl]
/// key  = "key.pem"
/// cert = "cert.pem"
/// port = 443
/// host = "localhost"
/// ```
#[derive(Serialize, Deserialize)]
pub struct ConfFile {
    port: u16,
    host: String,
    db: DBArgs,
    ssl: Option<SSLArgs>,
}
/// CONF: static immutable, just for read, can't write.
pub static CONF: Lazy<ConfFile> =
    LazyNew!(toml::from_slice(&std::fs::read("./conf.toml").unwrap()).unwrap());
/// Use these functions to get configurations inside CONF.
impl ConfFile {
    pub fn bind_args(&self) -> (&str, u16) {
        (&self.host, self.port)
    }
    pub fn ssl_config(&self) -> Option<((&str, u16), Result<SslAcceptorBuilder, ErrorStack>)> {
        self.ssl
            .as_ref()
            .map(|ssl| (ssl.ssl_bind_args(), ssl.ssl_builder()))
    }
    pub fn database_args(&self) -> &DBArgs {
        &self.db
    }
}

/// Use this function to initialize tracing.
pub fn tracing_config_initalize() {
    tracing_subscriber::registry()
        // register hourly_logger
        .with({
            fmt::layer()
                .pretty()
                .with_target(true)
                .with_line_number(true)
                .with_ansi(false)
                .with_writer(rolling::hourly("log", "tracelog_hourly.log"))
        })
        // register wholely_logger
        .with({
            fmt::layer()
                .pretty()
                .with_target(true)
                .with_line_number(true)
                .with_ansi(false)
                .with_writer(rolling::never("log", "tracelog_whole.log"))
        })
        // register console logger
        .with(fmt::layer().pretty().with_writer(std::io::stdout))
        // register open telemetry jaeger tracer
        .with({
            global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
            tracing_opentelemetry::layer().with_tracer(
                opentelemetry_jaeger::new_agent_pipeline()
                    .with_service_name("fleaxj")
                    .install_batch(opentelemetry::runtime::Tokio)
                    .unwrap(),
            )
        })
        // initialize the tracing subscriber
        .init();
}
