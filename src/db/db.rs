use refinery::embed_migrations;
use tokio_postgres::NoTls;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

embed_migrations!("migrations");

/// Connect to the database using `tokio-postgres` for migrations.
pub async fn connect_and_run_migrations(database_url: &str) -> Result<PgPool, Box<dyn std::error::Error>> {
    // Connect to the database using tokio-postgres
    let (mut client, connection) = tokio_postgres::connect(database_url, NoTls).await?;

    // Spawn a separate task for connection management
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Run migrations
    migrations::runner().run_async(&mut client).await?;
    println!("Migrations applied successfully!");

    // Create an sqlx connection pool for application logic
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}