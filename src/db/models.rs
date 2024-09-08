use uuid::Uuid;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Notification {
    pub user_id: Uuid,
    pub content: String,
    pub send_at: Option<OffsetDateTime>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub phone_number: Option<String>,
    pub email_verified: Option<bool>,
    pub verification_token: Option<Uuid>,
    pub phone_verified: Option<bool>,
    pub phone_verification_code: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}
