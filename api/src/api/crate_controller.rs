use crate::auth::User;
use crate::log_error_and_responde;
use crate::repository::cargo_repository::CargoRepository;
use crate::storage::Storage;
use actix_web::{delete, get, web, HttpResponse, Responder, Scope};

pub fn crate_controller() -> Scope {
    Scope::new("/crate")
        .service(get_crates)
        .service(delete_crate)
}

#[get("/{repo_id}")]
async fn get_crates(
    state: web::Data<CargoRepository>,
    repo_id: web::Path<i32>,
    _user: User,
) -> impl Responder {
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
    storage: web::Data<Box<dyn Storage>>,
    _user: User,
) -> impl Responder {
    let crate_id = crate_id.into_inner();

    let crates = match state.get_index_by_id(&crate_id).await {
        Ok(c) => c,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    match storage.remove(crates.path).await {
        Ok(_) => {}
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    match state.delete_crate(&crate_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => log_error_and_responde!(err),
    }
}
