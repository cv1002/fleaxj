use actix_web::{post, web, HttpResponse, Responder};
use sea_orm::ActiveModelTrait;
use serde::Deserialize;
use tracing::instrument;

use crate::active_model;
use crate::database::CONN_POLL;
use crate::util::{inspect::ResultInspectErr, try_do};

#[derive(Debug, Deserialize)]
pub struct Param {
    phone: String,
}
#[post("/hello")]
#[instrument(name = "hello-request")]
pub async fn hello(req: web::Query<Param>) -> impl Responder {
    let _ = try_do(|| async {
        active_model!(users {
            phone: req.into_inner().phone,
        })
        .insert(CONN_POLL.get().await)
        .await
        .inspect_err(|err| log::error!("{}", err.to_string()))
    })
    .await;
    HttpResponse::Ok().body("Hello world!")
}

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(actix_files::Files::new("/", "./static").use_last_modified(true));
}
