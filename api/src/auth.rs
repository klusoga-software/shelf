use crate::error::AuthError;
use crate::jwt::Claims;
use crate::repository::cargo_repository::CargoRepository;
use crate::repository::service_accounts_repository::ServiceAccountsRepository;
use actix_web::web::Data;
use actix_web::HttpRequest;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::env;

pub async fn check_auth(req: HttpRequest, required_permission: String) -> Result<(), AuthError> {
    if req.uri().path().contains("/cargo") && req.headers().get("Authorization").is_some() {
        let header = req.headers().get("Authorization").unwrap();
        let secret = env::var("JWT_SECRET").unwrap_or("secret".to_string());

        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["shelf"]);

        let claims = match decode::<Claims>(
            header.to_str().unwrap(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        ) {
            Ok(claim) => claim,
            Err(_) => return Err(AuthError::Unauthorized("invalid token".to_string())),
        };

        let service_account_repository = match req.app_data::<Data<ServiceAccountsRepository>>() {
            None => {
                return Err(AuthError::ActixDataMissing(
                    "ServiceAccountsRepository".to_string(),
                ))
            }

            Some(state) => state,
        };

        let cargo_repository = match req.app_data::<Data<CargoRepository>>() {
            None => return Err(AuthError::ActixDataMissing("CargoRepository".to_string())),
            Some(state) => state,
        };

        let repo_name = get_repository_name(req.uri().path());

        let repo = match cargo_repository.get_repo_by_name(&repo_name).await {
            Ok(repo) => repo,
            Err(_) => {
                return Err(AuthError::RepositoryNotFound(repo_name.to_string()));
            }
        };

        let permission = match service_account_repository
            .get_permissions(claims.claims.sub, repo.id.unwrap())
            .await
        {
            Ok(service_account) => service_account,
            Err(_) => {
                return Err(AuthError::Unauthorized("invalid token".to_string()));
            }
        };

        if !permission.contains(&required_permission) {
            return Err(AuthError::Unauthorized(
                "insufficient permissions".to_string(),
            ));
        }
    }
    Ok(())
}

fn get_repository_name(path: &str) -> String {
    let strings: Vec<&str> = path.split("/").collect();

    strings[2].to_string()
}
