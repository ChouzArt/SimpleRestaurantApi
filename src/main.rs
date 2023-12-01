mod domain;
mod presentation;
mod repository;
mod tests;
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::debug;
use presentation::{create_order, get_table_orders, delete_menu_item_from_order, get_order_from_menu_item_and_table};
use repository::PgSqlOrderRepository;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = setup_pg_db().await.unwrap();
    let pg_sql_order_repository = PgSqlOrderRepository::new(pool);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // Create a new order
            .route("/orders", web::post().to(create_order))
            // Read all orders from a table
            .route("/tables/{table_number}/orders", web::get().to(get_table_orders))
            // Read an order from a table and a menu item
            .route("/tables/{table_number}/menu_items/{menu_item_id}", web::get().to(get_order_from_menu_item_and_table))
            // Delete an order from a table
            .route("/tables/{table_number}/menu_items/{menu_item_id}", web::delete().to(delete_menu_item_from_order))
            // Add data to your app
            .app_data(web::Data::new(pg_sql_order_repository.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn setup_pg_db() -> Result<PgPool, sqlx::Error> {
    dotenvy::dotenv().ok(); // Loads the .env file with the test database secrets.
                            // Initialize logging
    let env_filter = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(env_filter));

    // Connect to the db
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(&database_url)
        .await?;
    debug!("Connecting to DB ... OK");

    Ok(pool)
}
