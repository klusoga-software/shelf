use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub ui: UiConfiguration,
    pub auth: AuthConfiguration,
    pub s3: Option<S3Configuration>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S3Configuration {
    pub host: String,
    pub bucket: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub insecure: bool,
}
