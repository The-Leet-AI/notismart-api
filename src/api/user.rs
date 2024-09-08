use actix_web::{web, HttpResponse};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;
use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use lettre::{SmtpTransport, Message, Transport};
use std::collections::HashMap;

use crate::db::models::User;  // Import user model

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateUser {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct UpdateUser {
    email: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserGet {
    pub id: Uuid,
    pub email: String,
    pub phone_number: Option<String>,  // Optional phone number field
    pub email_verified: Option<bool>,  // Track if the email is verified
    pub phone_verified: Option<bool>,  // Track if the phone number is verified
    pub created_at: Option<OffsetDateTime>,  // Timestamp for user creation
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    email: Option<String>,
    phone_number: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserLogin {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub email_verified: Option<bool>,
    pub phone_verified: Option<bool>,
    pub created_at: Option<OffsetDateTime>,
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct Claims {
    sub: Uuid,  // User ID
    exp: usize, // Expiration timestamp
}


// POST /users - Create a new user (register)
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User successfully created", body = String),
        (status = 400, description = "Bad request due to invalid input")
    ),
    tag = "User API"
)]
pub async fn create_user(
    user_data: web::Json<CreateUser>,  // Now, this holds the plain password
    db: web::Data<PgPool>,
) -> HttpResponse {
    let hashed_password = match hash(&user_data.password, DEFAULT_COST) {  // Hashing plain password
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().json("Error hashing password"),
    };

    let verification_token = Uuid::new_v4();  // Generate a token for email verification

    let result = sqlx::query!(
        "INSERT INTO users (email, password_hash, verification_token) VALUES ($1, $2, $3)",
        user_data.email,
        hashed_password,  // Store the hashed password
        verification_token
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => {
            // Send verification email with token
            let email = Message::builder()
                .from("ayoub.achak01@gmail.com".parse().unwrap())
                .to(user_data.email.parse().unwrap())
                .subject("Verify your email")
                .body(format!("Please verify your email by clicking the link: http://127.0.0.1:8080/verify?token={}", verification_token))
                .unwrap();

            let mailer = SmtpTransport::unencrypted_localhost();
            
            match mailer.send(&email) {
                Ok(_) => HttpResponse::Created().json("User created. Check your email for verification."),  // Use 201 status code
                Err(e) => HttpResponse::InternalServerError().json(format!("Failed to send verification email: {}", e)),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Error creating user"),
    }
}


// GET /users/{id} - Fetch a user by ID
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "User retrieved successfully", body = UserGet),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = Uuid, Path, description = "ID of the User to retrieve")
    ),
    tag = "User API"
)]
async fn get_user(
    user_id: web::Path<Uuid>,
    db: web::Data<PgPool>,
) -> HttpResponse {
    let user = sqlx::query_as!(
        UserGet,
        "SELECT id, email, phone_number, email_verified, phone_verified, created_at FROM users WHERE id = $1",
        user_id.into_inner()
    )
    .fetch_one(db.get_ref())
    .await;

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json("User not found"),
    }
}


// PUT /users/{id} - Update a user’s email or phone number
#[utoipa::path(
    put,
    path = "/api/users/{id}",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully"),
        (status = 500, description = "Error updating user")
    ),
    params(
        ("id" = Uuid, Path, description = "ID of the User to update")
    ),
    tag = "User API"
)]
async fn update_user(
    user_id: web::Path<Uuid>,
    user_data: web::Json<UpdateUserRequest>,
    db: web::Data<PgPool>,
) -> HttpResponse {
    let user_id_inner = user_id.into_inner();  // Move once, store in variable

    // Update email and phone_number in a single query, using COALESCE to preserve existing values if none provided
    let result = sqlx::query!(
        "UPDATE users SET email = COALESCE($1, email), phone_number = COALESCE($2, phone_number) WHERE id = $3",
        user_data.email,
        user_data.phone_number,
        user_id_inner
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("User updated"),
        Err(_) => HttpResponse::InternalServerError().json("Error updating user"),
    }
}


// DELETE /users/{id} - Delete a user by ID
#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 500, description = "Error deleting user")
    ),
    params(
        ("id" = Uuid, Path, description = "ID of the User to delete")
    ),
    tag = "User API"
)]
async fn delete_user(
    user_id: web::Path<Uuid>,
    db: web::Data<PgPool>,
) -> HttpResponse {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id.into_inner())
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("User deleted"),
        Err(_) => HttpResponse::InternalServerError().json("Error deleting user"),
    }
}


// POST /verify?token=<verification_token>
#[utoipa::path(
    post,
    path = "/api/verify",
    responses(
        (status = 200, description = "Email verified successfully"),
        (status = 400, description = "Invalid token format"),
        (status = 500, description = "Error verifying email")
    ),
    params(
        ("token" = String, Query, description = "Email verification token")
    ),
    tag = "User API"
)]
async fn verify_email(
    query: web::Query<HashMap<String, String>>,  // Token passed as a query param
    db: web::Data<PgPool>
) -> HttpResponse {
    if let Some(token_str) = query.get("token") {
        // Parse token from String to Uuid
        match Uuid::parse_str(token_str) {
            Ok(token) => {
                let result = sqlx::query!(
                    "UPDATE users SET email_verified = TRUE WHERE verification_token = $1",
                    token
                )
                .execute(db.get_ref())
                .await;

                match result {
                    Ok(_) => HttpResponse::Ok().json("Email verified successfully"),
                    Err(_) => HttpResponse::InternalServerError().json("Invalid or expired verification token"),
                }
            },
            Err(_) => {
                // Log or return a clear error message when the token format is invalid
                HttpResponse::BadRequest().json("Invalid token format")
            }
        }
    } else {
        HttpResponse::BadRequest().json("Verification token is missing")
    }
}


// POST /login - Authenticate a user, check if email is verified
#[utoipa::path(
    post,
    path = "/api/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Successful login", body = String),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "User API"
)]
async fn login(
    login_data: web::Json<LoginRequest>,
    db: web::Data<PgPool>,
) -> HttpResponse {
    let user = sqlx::query_as!(
        UserLogin,
        "SELECT id, email, password_hash, email_verified, phone_verified, created_at FROM users WHERE email = $1",
        login_data.email
    )
    .fetch_one(db.get_ref())
    .await;

    match user {
        Ok(user) => {
            // Check if email is verified
            if !user.email_verified.unwrap_or(false) {
                return HttpResponse::Unauthorized().json("Please verify your email before logging in");
            }

            // Verify the password
            if verify(&login_data.password, &user.password_hash).is_err() {
                return HttpResponse::Unauthorized().json("Invalid email or password");
            }

            // Generate JWT (assuming JWT generation logic here)
            let expiration = chrono::Utc::now();
            
            // Generate JWT (assuming JWT generation logic here)
            let expiration = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::hours(24))
                .expect("valid timestamp")
                .timestamp() as usize;

            let claims = Claims {
                sub: user.id,
                exp: expiration,
            };

            let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_ref()),  // Use the secret from env
            )
            .expect("Token creation failed");

            HttpResponse::Ok().json(token)
        }
        Err(_) => HttpResponse::Unauthorized().json("Invalid email or password"),
    }
}


// POST /resend-verification - Resend verification email
#[utoipa::path(
    post,
    path = "/api/resend-verification",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Verification email resent successfully"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Error sending verification email")
    ),
    tag = "User API"
)]
async fn resend_verification_email(
    user_data: web::Json<LoginRequest>,  // Use the email input from the user
    db: web::Data<PgPool>,
) -> HttpResponse {
    // Fetch the user from the database using their email
    let user = sqlx::query!(
        "SELECT id, email, email_verified FROM users WHERE email = $1",
        user_data.email
    )
    .fetch_one(db.get_ref())
    .await;

    match user {
        Ok(user) => {
            // Check if email is already verified
            if user.email_verified.unwrap_or(false) {
                return HttpResponse::Ok().json("Email is already verified");
            }

            // Generate a new verification token
            let new_verification_token = Uuid::new_v4();

            // Update the user's verification token in the database
            let result = sqlx::query!(
                "UPDATE users SET verification_token = $1 WHERE email = $2",
                new_verification_token,
                user_data.email
            )
            .execute(db.get_ref())
            .await;

            if result.is_err() {
                return HttpResponse::InternalServerError().json("Failed to update verification token");
            }

            // Send the new verification email
            let email = Message::builder()
                .from("noreply@yourapp.com".parse().unwrap())
                .to(user_data.email.parse().unwrap())
                .subject("Resend Verification Email")
                .body(format!("Please verify your email by clicking this link: http://yourapp.com/verify?token={}", new_verification_token))
                .unwrap();

            let mailer = SmtpTransport::unencrypted_localhost();  

            match mailer.send(&email) {
                Ok(_) => HttpResponse::Ok().json("Verification email resent successfully"),
                Err(_) => HttpResponse::InternalServerError().json("Failed to send verification email"),
            }
        }
        Err(_) => HttpResponse::NotFound().json("User not found"),
    }
}

// Initialize user-related routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user))     // POST /users
            .route("/{id}", web::get().to(get_user))    // GET /users/{id}
            .route("/{id}", web::put().to(update_user)) // PUT /users/{id}
            .route("/{id}", web::delete().to(delete_user)) // DELETE /users/{id}
    )
    .route("/login", web::post().to(login))  // POST /login
    .route("/verify", web::post().to(verify_email))  // POST /verify
    .route("/resend-verification", web::post().to(resend_verification_email));  // POST /resend-verification
}
