use crate::log_error_and_responde;
use crate::repository::cargo_repository::CargoRepository;
use actix_web::{delete, get, web, HttpResponse, Responder, Scope};

pub fn crate_controller() -> Scope {
    Scope::new("/crate")
        .service(get_crates)
        .service(delete_crate)
}

#[get("/{repo_id}")]
async fn get_crates(state: web::Data<CargoRepository>, repo_id: web::Path<i32>) -> impl Responder {
    let crates = match state.list_crates_for_repo(repo_id.into_inner()).await {
        Ok(crates) => crates,
        Err(err) => return log_error_and_responde!(err),
    };

    HttpResponse::Ok().json(crates)
}

#[delete("/{crate_id}")]
async fn delete_crate(
    state: web::Data<CargoRepository>,
    crate_id: web::Path<i32>,
) -> impl Responder {
    match state.delete_crate(crate_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => log_error_and_responde!(err),
    }
}
