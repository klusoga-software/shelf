use actix_web::Scope;
use crate::api::cargo::get_cargo_scope;
use crate::api::health_controller::get_health;
use crate::api::repo_controller::repo_controller;

pub mod cargo;
pub mod health_controller;
mod macros;
pub mod models;
pub mod repo_controller;

pub fn api_scope() -> Scope{
    Scope::new("/api")
        .service(get_health)
        .service(repo_controller())
        .service(get_cargo_scope())
}
