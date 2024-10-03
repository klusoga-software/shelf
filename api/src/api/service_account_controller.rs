use crate::api::models::CreateServiceAccount;
use crate::log_error_and_responde;
use crate::repository::service_accounts_repository::ServiceAccountsRepository;
use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse, Responder, Scope};

pub fn service_account_controller() -> Scope {
    Scope::new("/service-accounts")
        .service(list_service_accounts)
        .service(create_service_account)
}

#[get("")]
async fn list_service_accounts(state: web::Data<ServiceAccountsRepository>) -> impl Responder {
    match state.list_service_accounts().await {
        Ok(repos) => HttpResponse::Ok().json(repos),
        Err(err) => log_error_and_responde!(err),
    }
}

#[post("")]
async fn create_service_account(
    state: web::Data<ServiceAccountsRepository>,
    body: Json<CreateServiceAccount>,
) -> impl Responder {
    let secret: String = "secret".to_string();

    match state
        .create_service_account(body.into_inner(), &secret)
        .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => log_error_and_responde!(err),
    }
}
