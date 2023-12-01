use crate::repository::OrderRepository;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// Every item created for a table number is defined as a restaurant [Order]
#[derive(Debug, Clone)]
pub struct Order {
    pub id: Uuid,
    pub table_number: i32,
    pub menu_item_id: i32,
    pub created_at: DateTime<Utc>,
}

/// Defines the food item options on the menu. These need to be pre-registered in the database before taking new [Order]s.
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub id: i32,
    pub item_name: String,
    pub cooking_time: i32,
}

/// Struct to map complete queries that joins [MenuItem]s info into the [Order]s.
#[derive(Serialize, sqlx::FromRow, Debug, Clone)]
pub struct CompleteOrder {
    pub order_id: Uuid,
    pub table_number: i32,
    pub menu_item_id: i32,
    pub created_at: DateTime<Utc>,
    pub item_name: String,
    pub cooking_time: i32,
}

impl Order {
    /// Creates an order with a random UUID.
    ///
    /// [Self::menu_item_id] should be constrained to an existing [MenuItem::id] from the db table 'menu_items'.
    ///
    /// # Example
    /// ```
    /// let order = Order::new(1, 25);
    /// ```
    pub fn new(table_number: i32, menu_item_id: i32) -> Order {
        Order {
            id: Uuid::new_v4(),
            table_number,
            menu_item_id,
            created_at: chrono::offset::Utc::now(),
        }
    }

    /// Creates an order and inserts into the connected database.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let order = Order::new(1, 25);
    /// let uuid = order.create(&pg_sql_order_repository).await?;
    /// assert_eq!(4, uuid.get_version_num());
    /// ```
    pub async fn create<O: OrderRepository>(&self, repo: &O) -> Result<Uuid, O::ErrT> {
        let uuid = repo.create(&self).await?;
        Ok(uuid)
    }
}

/// Get all orders from a table number.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let orders: Vec<Order> = read_orders_by_table(&pg_sql_order_repository, table_number).await?;
/// assert!(orders.len() >= 0);
/// ```
pub async fn read_orders_by_table<O: OrderRepository>(
    repo: &O,
    table_number: i32,
) -> Result<Vec<CompleteOrder>, O::ErrT> {
    repo.read_orders_by_table(table_number).await
}

/// Get an order from a specific menu item and a specific table number.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let order = read_order_item_from_table(&pg_sql_order_repository, menu_item_id, table_num).await?;
/// assert!(order.is_some());
/// ```
pub async fn read_order_item_from_table<O: OrderRepository>(
    repo: &O,
    menu_item_id: i32,
    table_number: i32,
) -> Result<Option<CompleteOrder>, O::ErrT> {
    repo.read_order_item_from_table(menu_item_id, table_number)
        .await
}

/// Selects the latest created food item on a specified table and deletes it.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let rows_affected = repo.delete_order_item_from_table(menu_item_id, table_number).await?;
/// assert!(rows_affected >= 0);
/// ```
pub async fn delete_order_item_from_table<O: OrderRepository>(
    repo: &O,
    menu_item_id: i32,
    table_number: i32,
) -> Result<u64, O::ErrT> {
    repo.delete_order_item_from_table(menu_item_id, table_number)
        .await
}
