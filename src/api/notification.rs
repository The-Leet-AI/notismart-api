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
        (status = 200, description = "Notification successfully created", body = NotificationResponse),
        (status = 400, description = "Invalid input"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Notification API",
    security(
        ("BearerAuth" = [])
    )
)]
pub async fn create_notification(
    notification_data: web::Json<CreateNotification>,
    db: web::Data<PgPool>,
    auth_user: AuthenticatedUser,  // Bearer authentication
) -> HttpResponse {
    let user_id = match Uuid::parse_str(&notification_data.user_id) {
        Ok(uuid) => uuid,
        Err(_) => return invalid_uuid_response(),
    };

    let send_at = match parse_send_at(&notification_data.send_at) {
        Ok(datetime) => datetime,
        Err(err_response) => return err_response,
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
        Err(_) => internal_server_error(),
    }
}

fn parse_send_at(send_at_str: &Option<String>) -> Result<Option<OffsetDateTime>, HttpResponse> {
    match send_at_str {
        Some(date) => match OffsetDateTime::parse(date, &Rfc3339) {
            Ok(datetime) => Ok(Some(datetime)),
            Err(_) => Err(HttpResponse::BadRequest().json(NotificationResponse {
                success: false,
                message: "Invalid date format".to_string(),
                notification: None,
            })),
        },
        None => Ok(None),
    }
}

fn invalid_uuid_response() -> HttpResponse {
    HttpResponse::BadRequest().json(NotificationResponse {
        success: false,
        message: "Invalid UUID".to_string(),
        notification: None,
    })
}

fn internal_server_error() -> HttpResponse {
    HttpResponse::InternalServerError().json(NotificationResponse {
        success: false,
        message: "Failed to create notification".to_string(),
        notification: None,
    })
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/notifications", web::post().to(create_notification));
}
