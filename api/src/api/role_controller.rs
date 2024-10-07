use crate::auth::User;
use crate::log_error_and_responde;
use crate::repository::role_repository::RoleRepository;
use actix_web::{get, web, HttpResponse, Responder, Scope};

pub fn role_controller() -> Scope {
    Scope::new("/roles").service(get_roles)
}

#[get("")]
async fn get_roles(state: web::Data<RoleRepository>, user: User) -> impl Responder {
    match state.get_roles().await {
        Ok(roles) => HttpResponse::Ok().json(roles),
        Err(err) => log_error_and_responde!(err),
    }
}
