use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};
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

pub async fn ensure_schema(db: &DatabaseConnection) -> Result<(), DbErr> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS urls (
          id BIGINT AUTO_INCREMENT PRIMARY KEY,
          short_code VARCHAR(10) UNIQUE NOT NULL,
          original_url TEXT NOT NULL,
          created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
          clicks INT DEFAULT 0
        );
    "#;

    db.execute(Statement::from_string(db.get_database_backend(), sql.to_string()))
        .await?;

    tracing::info!("Database schema ensured");
    Ok(())
}
