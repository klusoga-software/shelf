use crate::api::cargo::crates::upload;
use crate::api::cargo::index::index;
use actix_web::web;
use auth::me;

pub mod auth;
pub mod index;

pub mod models;

pub mod crates;

pub fn get_cargo_scope() -> actix_web::Scope {
    web::scope("/cargo")
        .service(index())
        .service(upload)
        .service(me)
}
