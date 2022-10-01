use serde::Deserialize;
use serde::Serialize;

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
