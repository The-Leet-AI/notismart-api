use utoipa::{Modify, OpenApi, ToSchema};
use utoipa::openapi::{security::{HttpAuthScheme, HttpBuilder, SecurityScheme}, ObjectBuilder, Schema, SchemaFormat, SchemaType};
use utoipa::openapi::RefOr;
use uuid::Uuid;
use time::OffsetDateTime;
use crate::api::{user, notification};



#[derive(OpenApi)]
#[openapi(
    paths(
        user::create_user,
        user::login,
        user::get_user,
        user::update_user,
        user::delete_user,
        user::verify_email,
        notification::create_notification
    ),
    components(
        schemas(
            user::CreateUser, 
            user::LoginRequest, 
            user::UserGet, 
            user::UpdateUserRequest, 
            notification::CreateNotification, 
            notification::NotificationResponse, 
            crate::db::models::Notification,
            UuidSchema,
            OffsetDateTimeSchema
        )
    ),
    tags(
        (name = "User API", description = "User-related endpoints for account management, login, and registration."),
        (name = "Notification API", description = "Notification management endpoints.")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components.as_mut().unwrap().add_security_scheme(
            "BearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
/// Schema for representing UUID as a string
pub struct UuidSchema;

impl ToSchema<'_> for UuidSchema {
    fn schema() -> (&'static str, RefOr<Schema>) {
        (
            "Uuid",  // Name of the schema
            Schema::Object(
                ObjectBuilder::new()
                    .schema_type(SchemaType::String)
                    .format(Some(SchemaFormat::Custom("uuid".to_string())))
                    .build(),
            )
            .into(),
        )
    }
}

/// Schema for representing OffsetDateTime as a string
pub struct OffsetDateTimeSchema;

impl ToSchema<'_> for OffsetDateTimeSchema {
    fn schema() -> (&'static str, RefOr<Schema>) {
        (
            "OffsetDateTime",  // Name of the schema
            Schema::Object(
                ObjectBuilder::new()
                    .schema_type(SchemaType::String)
                    .format(Some(SchemaFormat::Custom("date-time".to_string())))
                    .build(),
            )
            .into(),
        )
    }
}
