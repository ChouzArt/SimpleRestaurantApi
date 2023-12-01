#[cfg(test)]
mod pg_sql_tests {
    use crate::domain::*;
    use crate::repository::PgSqlOrderRepository;
    use crate::constants::FOOD_ITEMS;
    use futures::future::try_join_all;
    use rand::Rng;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::PgPool;
    use std::env;
    use std::error::Error as stdErr;

    /// Connect, clean empty and set initial data
    async fn setup_pg_test_db() -> Result<PgPool, sqlx::Error> {
        dotenvy::dotenv().ok(); // Loads the .env file with the test database secrets.

        // Connect to the db
        let database_url = env::var("DATABASE_TEST_URL").expect("DATABASE_TEST_URL must be set");
        print!("Connecting to dev DB ...");
        let pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(30))
            .connect(&database_url)
            .await?;
        println!("OK");

        // Reset dev db
        sqlx::query!("TRUNCATE orders CASCADE")
            .execute(&pool)
            .await?;

        sqlx::query!("TRUNCATE menu_items CASCADE")
            .execute(&pool)
            .await?;
        println!(r#"Database reset ... OK"#,);

        // Populate food menu item types
        print!("Populating DB with food menu items ... ");
        let mut rng = rand::thread_rng();
        let mut futures = vec![];
        for (i, item) in FOOD_ITEMS.iter().enumerate() {
            futures.push(insert_menu_item(&pool, i, item, rng.gen_range(5..15)));
        }
        try_join_all(futures).await?;
        println!("OK");
        Ok(pool)
    }

    /// Helper function to insert a new food menu item into the database.
    async fn insert_menu_item(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: usize,
        item_name: &str,
        cooking_time: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO menu_items (id, item_name, cooking_time) VALUES ($1, $2, $3)",
            id as i32,
            item_name,
            cooking_time
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Test database CRUD functions:
    ///
    /// CREATE: order.
    ///
    /// READ: query all orders for a specified table number.
    /// 
    /// READ: query latest order for a specified menu item and for a specified table number.
    ///
    /// DELETE: latest order for a specified item and for a specified table number.
    #[actix_web::test]
    async fn test_create_read_delete_orders() -> Result<(), Box<dyn stdErr>> {
        println!("------test_create_read_delete_orders------");
        const TABLES_LEN: i32 = 100;
        const ORDERS_LEN: i32 = 50;

        let pool = setup_pg_test_db().await?;
        let pg_sql_order_repository = PgSqlOrderRepository::new(pool);
        let mut rng = rand::thread_rng();

        let orders = read_orders_by_table(&pg_sql_order_repository, 1).await?;
        assert_eq!(0, orders.len());

        print!("Testing CREATE and READ ... ");
        //CREATE and READ
        for table_num in 1..=TABLES_LEN {
            for _ in 1..=ORDERS_LEN {
                let order = Order::new(table_num, rng.gen_range(1..50));
                let uuid = order.create(&pg_sql_order_repository).await?;
                assert_eq!(4, uuid.get_version_num());
            }
            let orders = read_orders_by_table(&pg_sql_order_repository, table_num).await?;
            assert_eq!(ORDERS_LEN, orders.len() as i32);
            for order in orders {
                let order = read_order_item_from_table(&pg_sql_order_repository, order.menu_item_id, table_num).await?;
                assert!(order.is_some());
            }
        }
        println!("OK");

        print!("Testing DELETE ... ");
        //DELETE
        for table_num in 1..=TABLES_LEN {
            let orders: Vec<CompleteOrder> =
                read_orders_by_table(&pg_sql_order_repository, table_num).await?;
                assert_eq!(ORDERS_LEN, orders.len() as i32);
            for order in orders {
                let rows_affected = delete_order_item_from_table(
                    &pg_sql_order_repository,
                    order.menu_item_id,
                    order.table_number,
                )
                .await?;
                assert_eq!(1, rows_affected);
            }
        }
        println!("OK");

        Ok(())
    }

}
