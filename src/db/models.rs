
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
    pub phone_number: Option<String>,
    pub created_at: Option<OffsetDateTime>,   
}