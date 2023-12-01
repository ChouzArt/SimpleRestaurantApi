mod constants;
mod domain;
mod presentation;
mod repository;
mod tests;

use actix_web::{middleware::Logger, web, App, HttpServer};
use constants::FOOD_ITEMS;
use futures::future::try_join_all;
use log::{error, info};
use presentation::{
    create_order, delete_menu_item_from_order, get_order_from_menu_item_and_table, get_table_orders,
};
use rand::Rng;
use repository::PgSqlOrderRepository;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = match setup_pg_db().await {
        Ok(val) => val,
        Err(err) => {
            error!("{:?}", err);
            return Ok(());
        }
    };
    let pg_sql_order_repository = PgSqlOrderRepository::new(pool);
    let socket_addrs = env::var("SOCKETADDRS").expect("SOCKETADDRS must be set");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // Create a new order
            .route("/v1/orders", web::post().to(create_order))
            // Read all orders from a table
            .route(
                "/v1/tables/{table_number}/orders",
                web::get().to(get_table_orders),
            )
            // Read an order from a table and a menu item
            .route(
                "/v1/tables/{table_number}/menu_items/{menu_item_id}",
                web::get().to(get_order_from_menu_item_and_table),
            )
            // Delete an order from a table
            .route(
                "/v1/tables/{table_number}/menu_items/{menu_item_id}",
                web::delete().to(delete_menu_item_from_order),
            )
            // Add data to your app
            .app_data(web::Data::new(pg_sql_order_repository.clone()))
    })
    .bind(socket_addrs)?
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
    info!("Connecting to DB ... OK");

    // Run migrations
    sqlx::migrate!().run(&pool).await?;
    info!("Running migrations ... OK");

    // Reset food menu items table
    sqlx::query!("TRUNCATE menu_items CASCADE")
        .execute(&pool)
        .await?;
    let mut rng = rand::thread_rng();
    let mut futures = vec![];
    for (i, item) in FOOD_ITEMS.iter().enumerate() {
        futures.push(
            sqlx::query!(
                "INSERT INTO menu_items (id, item_name, cooking_time) VALUES ($1, $2, $3)",
                i as i32,
                item,
                rng.gen_range(5..15),
            )
            .execute(&pool),
        );
    }
    try_join_all(futures).await?;
    info!("Menu items table RESET ... OK");

    Ok(pool)
}
