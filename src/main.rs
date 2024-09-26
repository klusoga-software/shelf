use crate::api::cargo::get_cargo_scope;
use crate::controller::health_controller::get_health;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{from_fn, Logger, Next};
use actix_web::{App, Error, HttpServer};
use env_logger::Env;

mod controller;

mod error;

mod api;

async fn auth(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    next.call(req).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(from_fn(auth))
            .service(get_health)
            .service(get_cargo_scope())
    })
    .bind(("0.0.0.0", 6300))?
    .run()
    .await
}
