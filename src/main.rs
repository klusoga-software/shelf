use crate::api::cargo::get_cargo_scope;
use crate::controller::health_controller::get_health;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;

mod controller;

mod error;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(get_health)
            .service(get_cargo_scope())
    })
    .bind(("0.0.0.0", 6300))?
    .run()
    .await
}
