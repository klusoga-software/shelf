use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceAccountClaims {
    pub exp: usize,
    pub key: String,
    pub aud: String,
    pub sub: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiClaims {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use crate::jwt::ServiceAccountClaims;
    use chrono::{DateTime, Utc};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use std::str::FromStr;

    #[test]
    fn token_validation() {
        let dt = DateTime::<Utc>::from_str("1970-01-01T00:00:00Z").unwrap();

        let timestamp = dt.timestamp() as usize;

        let claims = ServiceAccountClaims {
            exp: timestamp,
            key: "internal".to_string(),
            aud: "shelf".to_string(),
            sub: 1,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(b"secret"),
        )
        .unwrap();

        assert_eq!(token, "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjAsImtleSI6ImludGVybmFsIiwiYXVkIjoic2hlbGYiLCJzdWIiOjF9.D3RZVPYdA1N_6kmrmBvRFBICDLEvwui4pTqQjebBnNs");
    }
}
