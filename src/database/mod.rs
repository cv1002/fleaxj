pub mod model;
pub mod extend;

use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::{conf::{DBArgs, CONF}, util::lazy::AsyncLazy, AsyncLazyNew};

pub async fn establish_connection() -> DatabaseConnection {
    let DBArgs {
        user,
        host,
        database,
        password,
        ..
    } = CONF.database_args();
    let opt = ConnectOptions::new(format!("mysql://{user}:{password}@{host}/{database}"))
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Trace)
        .clone();
    Database::connect(opt).await.unwrap()
}

pub static CONN_POLL: AsyncLazy<DatabaseConnection> =
    AsyncLazyNew!(establish_connection());

#[macro_export]
macro_rules! active_model {
    ($name:ident { $($id:ident: $expr:expr),* $(,)? }) => {
        crate::database::model::$name::ActiveModel { $($id: sea_orm::Set($expr)),* , ..Default::default() }
    };
}