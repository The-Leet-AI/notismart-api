
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;


#[derive(Serialize, Deserialize)]
pub struct Notification {
    pub user_id: Uuid,  // Change to Uuid
    pub content: String,
    pub send_at: Option<OffsetDateTime>,
}


#[derive(Serialize, Deserialize)]
pub struct CreateNotification {
    pub user_id: String,
    pub content: String,
    pub send_at: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub phone_number: Option<String>,  // Optional phone number field
    pub email_verified: Option<bool>,  // Track if the email is verified
    pub verification_token: Option<Uuid>,  // Token for email verification
    pub phone_verified: Option<bool>,  // Track if the phone number is verified
    pub phone_verification_code: Option<String>,  // Code for phone verification (optional)
    pub created_at: Option<OffsetDateTime>,  // Timestamp for user creation
}