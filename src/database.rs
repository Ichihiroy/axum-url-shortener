use sea_orm::{Database, DatabaseConnection, DbErr};
use std::time::Duration;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    tracing::info!("Connecting to database...");
    
    let db = Database::connect(&database_url).await?;
    
    // Set connection pool settings
    let mut opt = sea_orm::ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);

    tracing::info!("Database connection established successfully");
    
    Ok(db)
}

pub async fn test_connection(db: &DatabaseConnection) -> Result<(), DbErr> {
    db.ping().await?;
    tracing::info!("Database ping successful");
    Ok(())
}
