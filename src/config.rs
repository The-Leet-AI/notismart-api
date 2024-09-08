use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_server: String,
    pub smtp_port: u16,
}

pub fn load_config() -> Config {
    dotenv().ok();

    Config {
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        smtp_username: env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set"),
        smtp_password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
        smtp_server: env::var("SMTP_SERVER").expect("SMTP_SERVER must be set"),
        smtp_port: env::var("SMTP_PORT").expect("SMTP_PORT must be set").parse().expect("Invalid SMTP_PORT"),
    }
}
