mod handlers;
mod models;
mod routes;

mod db {
    pub mod db;
}

use db::db::connect_and_run_migrations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "postgres://shopify:shopify@localhost:5432/sprint";

    // Connect to the database and run migrations
    let pool = connect_and_run_migrations(database_url).await?;

    let app = routes::app_routes(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
