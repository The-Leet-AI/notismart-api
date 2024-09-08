use dotenv::dotenv;
use std::env;

pub fn load_config() {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}
