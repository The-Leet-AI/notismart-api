use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::db::models::Notification;
use crate::services::notification;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use crate::auth::extractor::AuthenticatedUser;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateNotification {
    pub user_id: String,
    pub content: String,
    pub send_at: Option<String>,
}

#[derive(ToSchema, Serialize)]
pub struct NotificationResponse {
    pub success: bool,
    pub message: String,
    pub notification: Option<Notification>,
}

#[utoipa::path(
    post,
    path = "/api/notifications",
    request_body = CreateNotification,
    responses(
        (status = 200, description = "Notification created", body = NotificationResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn create_notification(
    notification_data: web::Json<CreateNotification>,
    db: web::Data<PgPool>,
    auth_user: AuthenticatedUser  // Bearer authentication
) -> HttpResponse {
    let user_id = match Uuid::parse_str(&notification_data.user_id) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().json(NotificationResponse {
            success: false,
            message: "Invalid UUID".to_string(),
            notification: None,
        }),
    };

    let send_at = match &notification_data.send_at {
        Some(send_at_str) => {
            match OffsetDateTime::parse(send_at_str, &Rfc3339) {
                Ok(datetime) => Some(datetime),
                Err(_) => return HttpResponse::BadRequest().json(NotificationResponse {
                    success: false,
                    message: "Invalid date format".to_string(),
                    notification: None,
                }),
            }
        }
        None => None,
    };

    let new_notification = Notification {
        user_id,
        content: notification_data.content.clone(),
        send_at,
    };

    match notification::create_notification(db.get_ref(), new_notification.clone()).await {
        Ok(_) => HttpResponse::Ok().json(NotificationResponse {
            success: true,
            message: "Notification created".to_string(),
            notification: Some(new_notification),
        }),
        Err(_) => HttpResponse::InternalServerError().json(NotificationResponse {
            success: false,
            message: "Failed to create notification".to_string(),
            notification: None,
        }),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/notifications", web::post().to(create_notification));
}
