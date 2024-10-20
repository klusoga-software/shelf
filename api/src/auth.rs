use crate::configuration::{Configuration, OidcConfiguration};
use crate::error::AuthError;
use crate::jwt::{ApiClaims, ServiceAccountClaims};
use crate::repository::cargo_repository::CargoRepository;
use crate::repository::service_accounts_repository::ServiceAccountsRepository;
use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{Error, FromRequest, HttpRequest};
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use std::env;
use std::future::Future;
use std::pin::Pin;

/// Used to verify service account jwt.
/// It will extract the auth header directly from the HttpRequest
///
/// # Arguments
///
/// * `req`: The http request out of the http handler
/// * `required_permission`: The string of the minimum requirements that are needed
///
/// returns: Result<(), AuthError>
///
/// # Examples
///
/// ```
/// match check_package_auth(req, "W".to_string()).await {
///     Ok(_) => {}
///     Err(err) => {
///         return match err {
///             AuthError::Unauthorized(message) => HttpResponse::Unauthorized().body(message),
///             AuthError::ActixDataMissing(message) => {
///                 HttpResponse::InternalServerError().body(message)
///             }
///             AuthError::RepositoryNotFound(repo) => HttpResponse::NotFound().body(repo),
///         }
///     }
/// }
/// ```
pub async fn check_package_auth(
    req: HttpRequest,
    required_permission: String,
) -> Result<(), AuthError> {
    if req.uri().path().contains("/cargo") && req.headers().get("Authorization").is_some() {
        let header = req.headers().get("Authorization").unwrap();
        let secret = env::var("JWT_SECRET").unwrap_or("secret".to_string());

        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["shelf"]);

        let claims = match decode::<ServiceAccountClaims>(
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

#[derive(Debug)]
pub struct User {
    pub claims: ApiClaims,
}

impl FromRequest for User {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<User, Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        let configuration = match req.app_data::<Data<Configuration>>() {
            None => {
                return Box::pin(async {
                    Err(actix_web::error::ErrorInternalServerError(
                        "internal server error",
                    ))
                })
            }
            Some(config) => config.clone(),
        };

        let oidc_config_url = configuration.ui.oidc_configuration_url.clone();

        Box::pin(async move {
            let auth_header = match req.headers().get("Authorization") {
                None => return Err(actix_web::error::ErrorUnauthorized("invalid token")),
                Some(token) => token,
            };

            let token = match auth_header.to_str() {
                Ok(header) => header.trim_start_matches("Bearer ").to_string(),
                Err(err) => return Err(actix_web::error::ErrorInternalServerError(err)),
            };

            let token_header =
                decode_header(&token).map_err(actix_web::error::ErrorUnauthorized)?;

            let response: OidcConfiguration = reqwest::get(oidc_config_url)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?
                .json()
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

            let response: JwkSet = reqwest::get(response.jwks_uri)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?
                .json()
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

            let key = DecodingKey::from_jwk(response.find(&token_header.kid.unwrap()).unwrap())
                .map_err(actix_web::error::ErrorInternalServerError)?;

            let mut validation = Validation::new(token_header.alg);
            validation.set_audience(&[&configuration.auth.audience]);
            validation.set_issuer(&[&configuration.auth.issuer]);

            let claim = decode::<ApiClaims>(&token, &key, &validation)
                .map_err(actix_web::error::ErrorUnauthorized)?;

            Ok(User {
                claims: claim.claims,
            })
        })
    }
}
