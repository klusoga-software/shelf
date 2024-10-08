use crate::api::cargo::crates::{download, upload};
use crate::api::cargo::index::{config, index_files};
use actix_web::web;
use auth::me;

pub mod auth;
pub mod crates;
pub mod index;
pub mod models;

pub fn get_cargo_scope() -> actix_web::Scope {
    web::scope("/cargo")
        .service(config)
        .service(me)
        .service(index_files)
        .service(upload)
        .service(download)
}
