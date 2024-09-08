use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
}

pub fn load_config() -> Config {
    dotenv().ok();
    
    Config {
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
    }
}
