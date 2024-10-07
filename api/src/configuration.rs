use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub ui: UiConfiguration,
    pub auth: AuthConfiguration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UiConfiguration {
    pub oidc_configuration_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfiguration {
    pub audience: String,
    pub issuer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OidcConfiguration {
    pub issuer: String,
    pub jwks_uri: String,
}
