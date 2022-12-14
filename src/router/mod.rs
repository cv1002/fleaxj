/// Use of actix_web
use actix_web::{get, post, web, HttpResponse, Responder};
/// Use of sea_orm
use sea_orm::ActiveModelTrait;
/// Use of serde
use serde::Deserialize;
/// Use of tracing
use tracing::instrument;

/// Use of other module
use crate::active_model;
use crate::database::CONN_POLL;
use crate::util::{inspect::ResultInspectErr, try_do};

#[derive(Debug, Deserialize)]
pub struct Param {
    phone: String,
}
#[post("/hello")]
#[instrument(name = "hello-request")]
pub async fn hello(req: web::Json<Param>) -> impl Responder {
    // 业务逻辑
    try_do(|| async {
        active_model!(users {
            phone: req.into_inner().phone,
        })
        .insert(CONN_POLL.get().await)
        .await
        .inspect_error(|err| log::error!("{}", err.to_string()))
    })
    .await
    // 成功逻辑
    .map(|_| HttpResponse::Ok().body("Ok"))
    // 失败逻辑
    .unwrap_or_else(|_| HttpResponse::InternalServerError().body("Failed"))
}

#[get("/hi")]
#[instrument(name = "hi-request")]
pub async fn hi(req: web::Query<Param>) -> impl Responder {
    // 业务逻辑
    try_do(|| async {
        active_model!(users {
            phone: req.into_inner().phone,
        })
        .insert(CONN_POLL.get().await)
        .await
        .inspect_error(|err| log::error!("{}", err.to_string()))
    })
    .await
    // 成功逻辑
    .map(|_| HttpResponse::Ok().body("Ok"))
    // 失败逻辑
    .unwrap_or_else(|_| HttpResponse::InternalServerError().body("Failed"))
}

#[get("/")]
#[instrument(name = "index-request")]
pub async fn index() -> impl Responder {
    actix_files::NamedFile::open_async("./static/index.html").await
}

/// Use this function to initialize routers
pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(hi)
        .service(index)
        .service(actix_files::Files::new("/", "./static"));
}
