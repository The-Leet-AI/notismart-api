pub mod models;

use sqlx::postgres::PgPoolOptions;

pub async fn connect(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    
    Ok(pool)
}

