use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Validation, DecodingKey, TokenData};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub sub: Uuid,  // User ID (Subject)
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = extract_bearer_token(auth_str) {
                    return decode_jwt(token);
                }
            }
        }

        err(actix_web::error::ErrorUnauthorized("Invalid or missing JWT token"))
    }
}

fn extract_bearer_token(auth_str: &str) -> Option<&str> {
    if auth_str.starts_with("Bearer ") {
        Some(auth_str.trim_start_matches("Bearer ").trim())
    } else {
        None
    }
}

fn decode_jwt(token: &str) -> Ready<Result<AuthenticatedUser, actix_web::Error>> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    match decode::<AuthenticatedUser>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()) {
        Ok(data) => ok(data.claims),
        Err(_) => err(actix_web::error::ErrorUnauthorized("Invalid or expired JWT token")),
    }
}
