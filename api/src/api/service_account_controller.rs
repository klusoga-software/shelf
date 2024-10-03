use crate::log_error_and_responde;
use crate::repository::service_accounts_repository::ServiceAccountsRepository;
use actix_web::{get, web, HttpResponse, Responder, Scope};
use sqlx::Error;

pub fn service_account_controller() -> Scope {
    Scope::new("/service-accounts").service(list_service_accounts)
}

#[get("")]
async fn list_service_accounts(state: web::Data<ServiceAccountsRepository>) -> impl Responder {
    match state.list_service_accounts().await {
        Ok(repos) => HttpResponse::Ok().json(repos),
        Err(err) => log_error_and_responde!(err),
    }
}
