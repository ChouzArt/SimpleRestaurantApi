mod constants;
mod domain;
pub mod presentation;
pub mod repository;
mod tests;

use actix_web::{dev::Server, middleware::Logger, web, App, Error, HttpServer};
use futures::future::try_join_all;
use log::info;
use presentation::*;
use rand::Rng;
use repository::PgSqlOrderRepository;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

/// Creates new HTTP server with a PostgreSQL database connection.
pub async fn new_http_pg_server(
    socket_addrs: &str,
    repo: PgSqlOrderRepository,
) -> Result<Server, Error> {
    let server = HttpServer::new(move || {
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
            .route(
                "/v1/orders/{order_id}",
                web::delete().to(delete_order),
            )
            // Add data to your app
            .app_data(web::Data::new(repo.clone()))
    })
    .bind(socket_addrs)?
    .run();
    Ok(server)
}

pub async fn setup_pg_db() -> Result<PgPool, sqlx::Error> {
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

    // Initialize the restaurant's menu_items TABLE if first time running this DB.
    let rows = sqlx::query!("SELECT * FROM menu_items")
        .fetch_all(&pool)
        .await?;
    match rows.len() >= 50 {
        true => {
            info!("TABLE menu_items already initialized ... OK");
            return Ok(pool);
        }
        false => {
            sqlx::query!("TRUNCATE menu_items CASCADE")
                .execute(&pool)
                .await?;
            let mut rng = rand::thread_rng();
            let mut futures = vec![];
            for (i, item) in constants::FOOD_ITEMS.iter().enumerate() {
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
            info!("TABLE menu_items initialized ... OK");
            return Ok(pool);
        }
    }
}
