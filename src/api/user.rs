use actix_web::{web, HttpResponse};
use uuid::Uuid;
use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};

use crate::db::models::User;  // Import user model

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct UpdateUser {
    email: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct Claims {
    sub: Uuid,  // User ID
    exp: usize, // Expiration timestamp
}

// POST /users - Create a new user (register)
async fn create_user(
    user_data: web::Json<CreateUser>,
    db: web::Data<PgPool>,
) -> HttpResponse {
    let hashed_password = match hash(&user_data.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().json("Error hashing password"),
    };

    let result = sqlx::query!(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2)",
        user_data.email,
        hashed_password
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("User created"),
        Err(_) => HttpResponse::InternalServerError().json("Error creating user"),
    }
}

// GET /users/{id} - Fetch a user by ID
async fn get_user(
    user_id: web::Path<Uuid>,
    db: web::Data<PgPool>,
) -> HttpResponse {

    let result = sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, phone_number, created_at FROM users WHERE id = $1",
        user_id.into_inner()
    )
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json("User not found"),
    }
}

// PUT /users/{id} - Update a user’s email
async fn update_user(
    user_id: web::Path<Uuid>,
    user_data: web::Json<UpdateUser>,
    db: web::Data<PgPool>,
) -> HttpResponse {
    let result = sqlx::query!(
        "UPDATE users SET email = $1 WHERE id = $2",
        user_data.email,
        user_id.into_inner()
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("User updated"),
        Err(_) => HttpResponse::InternalServerError().json("Error updating user"),
    }
}

// DELETE /users/{id} - Delete a user by ID
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


// POST /login - Authenticate a user
async fn login(
    login_data: web::Json<LoginRequest>,
    db: web::Data<PgPool>,
) -> HttpResponse {
    let user = match sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, phone_number, created_at FROM users WHERE email = $1",  // Add phone_number
        login_data.email
    )
    .fetch_one(db.get_ref())
    .await
    {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().json("Invalid email or password"),
    };

    // Verify password
    if verify(&login_data.password, &user.password_hash).is_err() {
        return HttpResponse::Unauthorized().json("Invalid email or password");
    }

    // Generate JWT
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("your_secret_key".as_ref()),  // Replace with an actual secret
    )
    .expect("token should be created");

    HttpResponse::Ok().json(token)
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
    .route("/login", web::post().to(login));  // POST /login
}
