use crate::api::configuration::get_configuration;
use crate::api::crate_controller::crate_controller;
use crate::api::health_controller::get_health;
use crate::api::repo_controller::repo_controller;
use actix_web::Scope;

pub mod cargo;
pub mod crate_controller;
pub mod health_controller;
mod macros;
pub mod models;
pub mod repo_controller;

pub mod configuration;

pub fn api_scope() -> Scope {
    Scope::new("/api")
        .service(get_health)
        .service(repo_controller())
        .service(crate_controller())
        .service(get_configuration)
}
