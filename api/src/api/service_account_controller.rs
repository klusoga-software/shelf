use crate::api::models::CreateServiceAccount;
use crate::jwt::Claims;
use crate::log_error_and_responde;
use crate::repository::service_accounts_repository::ServiceAccountsRepository;
use actix_web::web::Json;
use actix_web::{delete, get, post, web, HttpResponse, Responder, Scope};
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use std::str::FromStr;

pub fn service_account_controller() -> Scope {
    Scope::new("/service-accounts")
        .service(list_service_accounts)
        .service(create_service_account)
        .service(delete_service_account)
}

#[get("")]
async fn list_service_accounts(state: web::Data<ServiceAccountsRepository>) -> impl Responder {
    match state.list_service_accounts().await {
        Ok(service_accounts) => HttpResponse::Ok().json(service_accounts),
        Err(err) => log_error_and_responde!(err),
    }
}

#[post("")]
async fn create_service_account(
    state: web::Data<ServiceAccountsRepository>,
    body: Json<CreateServiceAccount>,
) -> impl Responder {
    let create_service_account = body.into_inner();

    match state.create_service_account(&create_service_account).await {
        Ok(id) => {
            let expired_at = create_service_account
                .expired_at
                .unwrap_or(DateTime::<Utc>::from_str("2999-01-01T00:00:00Z").unwrap());

            let claims = Claims {
                key: create_service_account.name.clone(),
                aud: "shelf".to_string(),
                exp: expired_at.timestamp() as usize,
                sub: id,
            };

            let jwt_secret = std::env::var("JWT_SECRET").unwrap_or("secret".to_string());

            let token = match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(jwt_secret.as_bytes()),
            ) {
                Ok(token) => token,
                Err(err) => return log_error_and_responde!(err),
            };

            HttpResponse::Created().json(json!({"secret": token.clone()}))
        }
        Err(err) => log_error_and_responde!(err),
    }
}

#[delete("/{id}")]
async fn delete_service_account(
    state: web::Data<ServiceAccountsRepository>,
    id: web::Path<i32>,
) -> impl Responder {
    match state.delete_service_account(id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => log_error_and_responde!(err),
    }
}
