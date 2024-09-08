use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::db::models::{CreateNotification, Notification};  // Import models
use crate::services::notification;  // Import the service function
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;


async fn create_notification(
    notification_data: web::Json<CreateNotification>,
    db: web::Data<sqlx::PgPool>,
) -> HttpResponse {
    // Convert user_id from String to Uuid
    let user_id = match Uuid::parse_str(&notification_data.user_id) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid UUID"),
    };

    // Convert send_at from Option<String> to Option<OffsetDateTime>
    let send_at = match &notification_data.send_at {
        Some(send_at_str) => {
            match OffsetDateTime::parse(send_at_str, &Rfc3339) {
                Ok(datetime) => Some(datetime),
                Err(_) => return HttpResponse::BadRequest().json("Invalid date format"),
            }
        }
        None => None,
    };

    // Create a new Notification with the converted types
    let new_notification = Notification {
        user_id,
        content: notification_data.content.clone(),
        send_at,  // Use the converted Option<OffsetDateTime>
    };

    // Call the service layer function, passing the new_notification
    match notification::create_notification(db.get_ref(), new_notification).await {
        Ok(_) => HttpResponse::Ok().json("Notification created"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create notification"),
    }
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/notifications", web::post().to(create_notification));
}
