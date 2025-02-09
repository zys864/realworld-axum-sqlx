use crate::error::ErrorKind;
use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});
/// jwt auth

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// email
    pub sub: String,
    pub exp: i64,
}
impl Claims {
    pub fn new(email: String) -> Self {
        let iat = chrono::Utc::now();
        let exp = iat + chrono::Duration::hours(24);

        Self {
            sub: email,
            exp: chrono::DateTime::timestamp(&exp),
        }
    }
}
pub fn generate_jwt_token(claims: Claims) -> jsonwebtoken::errors::Result<String> {
    // Create the authorization token
    jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ErrorKind;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    jsonwebtoken::errors::Error::from(
                        jsonwebtoken::errors::ErrorKind::InvalidToken,
                    )
                })?;
        // Decode the user data
        let token_data =
            decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())?;

        Ok(token_data.claims)
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
