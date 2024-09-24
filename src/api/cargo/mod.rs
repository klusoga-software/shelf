use crate::api::cargo::index::index;
use actix_web::web;

pub mod auth;
pub mod index;

pub mod models;

pub fn get_cargo_scope() -> actix_web::Scope {
    web::scope("/cargo").service(index())
}
