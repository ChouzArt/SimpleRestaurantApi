use restaurant::repository::PgSqlOrderRepository;
use restaurant::{setup_pg_db, new_http_pg_server};
use std::env;
use log::error;


#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    // Loads the environment variables
    // - Local dev loads from .env
    // - Container loads from .yml file
    dotenvy::dotenv().ok(); 

    // Initialize logging
    let env_filter = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env_logger::init_from_env(env_logger::Env::default().default_filter_or(env_filter));

    // Load the asynchronous pool of SQLx database connections.
    let pool = match setup_pg_db().await {
        Ok(val) => val,
        Err(err) => {
            error!("{:?}", err);
            return Ok(());
        }
    };

    // Run the http server using a pgsql db
    let socket_addrs = env::var("SOCKETADDRS").expect("SOCKETADDRS must be set");
    let pg_sql_order_repository = PgSqlOrderRepository::new(pool);
    let server_result = new_http_pg_server(&socket_addrs, pg_sql_order_repository).await;
    match server_result {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}