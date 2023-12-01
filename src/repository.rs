use crate::domain::{CompleteOrder, Order};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait OrderRepository {
    type ErrT;

    /// CREATE - Store the item [Order] with the table number and how long the item will take to cook.
    async fn create(&self, order: &Order) -> Result<Uuid, Self::ErrT>;

    /// READ - Show all [Order] items for a specified table number
    async fn read_orders_by_table(
        &self,
        table_number: i32,
    ) -> Result<Vec<CompleteOrder>, Self::ErrT>;

    /// READ - Query latest [Order] item for a specified menu item [Order::menu_item_id] for a specified table number.
    async fn read_order_item_from_table(
        &self,
        menu_item_id: i32,
        table_number: i32,
    ) -> Result<Option<CompleteOrder>, Self::ErrT>;

    /// UPDATE - Not implemented. For this simple API, updates are done by removing and creating new [Order]s.
    async fn update_order(&self) -> Result<(), Self::ErrT>;

    /// DELETE - Remove latest [Order] item for a specified menu item [Order::menu_item_id] for a specified table number.
    async fn delete_order_item_from_table(
        &self,
        menu_item_id: i32,
        table_number: i32,
    ) -> Result<u64, Self::ErrT>;
}

#[derive(Clone)]
pub struct PgSqlOrderRepository {
    pool: PgPool,
}

impl PgSqlOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        PgSqlOrderRepository { pool }
    }
}

#[async_trait]
impl OrderRepository for PgSqlOrderRepository {
    type ErrT = sqlx::Error;

    async fn create(&self, order: &Order) -> Result<Uuid, Self::ErrT> {
        sqlx::query!(
            "INSERT INTO orders (id, table_number, menu_item_id, created_at) VALUES ($1, $2, $3, $4)",
            order.id,
            order.table_number,
            order.menu_item_id,
            order.created_at,
        )
        .execute(&self.pool)
        .await?;
        Ok(order.id)
    }

    async fn read_orders_by_table(
        &self,
        table_number: i32,
    ) -> Result<Vec<CompleteOrder>, Self::ErrT> {
        sqlx::query_as!(
            CompleteOrder,
            r#"SELECT orders.id as "order_id", table_number, menu_item_id, created_at, item_name, cooking_time
            FROM orders
            INNER JOIN menu_items ON orders.menu_item_id = menu_items.id
            WHERE orders.table_number = $1
            "#, table_number
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn read_order_item_from_table(
        &self,
        menu_item_id: i32,
        table_number: i32,
    ) -> Result<Option<CompleteOrder>, Self::ErrT> {
        sqlx::query_as!(
            CompleteOrder,
            r#"SELECT orders.id as "order_id", table_number, menu_item_id, created_at, item_name, cooking_time
            FROM orders
            INNER JOIN menu_items ON orders.menu_item_id = menu_items.id
            WHERE orders.menu_item_id = $1 AND orders.table_number = $2
            "#, 
            menu_item_id,
            table_number
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn update_order(&self) -> Result<(), Self::ErrT> {
        Ok(())
    }

    async fn delete_order_item_from_table(
        &self,
        menu_item_id: i32,
        table_number: i32,
    ) -> Result<u64, Self::ErrT> {
        let rows_deleted = sqlx::query!(
            "DELETE FROM orders WHERE id IN (
              SELECT id FROM orders
              WHERE table_number = $1 AND menu_item_id = $2
              ORDER BY created_at DESC LIMIT 1
             )",
            table_number,
            menu_item_id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        Ok(rows_deleted)
    }
}
