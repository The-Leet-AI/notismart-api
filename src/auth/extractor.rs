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
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.trim_start_matches("Bearer ").trim();
                    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

                    let token_data: Result<TokenData<AuthenticatedUser>, _> = decode::<AuthenticatedUser>(
                        token,
                        &DecodingKey::from_secret(secret.as_ref()),
                        &Validation::default(),
                    );

                    if let Ok(data) = token_data {
                        return ok(data.claims);  // Return the valid AuthenticatedUser
                    }
                }
            }
        }

        // Return the unauthorized error directly as Err
        err(actix_web::error::ErrorUnauthorized("Invalid or missing JWT token"))
    }
}
