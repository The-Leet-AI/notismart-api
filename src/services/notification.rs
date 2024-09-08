use crate::db::models::Notification;
use log::{error, info};
use sqlx::PgPool;

pub async fn create_notification(
    pool: &PgPool,
    notification: Notification,
) -> Result<(), sqlx::Error> {
    info!("Creating notification for user: {}", notification.user_id);

    let result = sqlx::query!(
        "INSERT INTO notifications (user_id, content, send_at, status) 
         VALUES ($1, $2, $3, 'Pending')",
        notification.user_id,
        notification.content,
        notification.send_at
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            info!("Notification created successfully for user: {}", notification.user_id);
            Ok(())
        }
        Err(e) => {
            error!("Failed to create notification for user {}: {:?}", notification.user_id, e);
            Err(e)
        }
    }
}
