use crate::domain::{
    delete_order_item_from_table, read_order_item_from_table, read_orders_by_table, Order,
};
use crate::repository::PgSqlOrderRepository;
use actix_web::{web, HttpResponse};
use log::error;
use serde::Deserialize;

/// The definition of [OrderRequest] which captures incoming JSON data
#[derive(Deserialize)]
pub struct OrderRequest {
    pub table_number: i32,
    pub menu_item_id: i32,
}

/// Post handler for creating an item [Order] with the table number and how long the item will take to cook.
pub async fn create_order(
    data: web::Data<PgSqlOrderRepository>,
    form: web::Json<OrderRequest>,
) -> HttpResponse {
    let order = Order::new(form.table_number, form.menu_item_id)
        .create(data.as_ref())
        .await;
    match order {
        Ok(uuid) => HttpResponse::Ok().json(uuid),
        Err(error) => {
            error!("{:?}", error);
            match error {
                sqlx::Error::Database(_) => {
                    HttpResponse::InternalServerError().json("This menu item doesn't exist.")
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

/// Get handler for querying all [Order] items for a specified table number.
pub async fn get_table_orders(
    data: web::Data<PgSqlOrderRepository>,
    path: web::Path<i32>,
) -> HttpResponse {
    let table_num = path.into_inner();
    let orders = read_orders_by_table(data.as_ref(), table_num).await;
    match orders {
        Ok(items) if items.len() > 0 => HttpResponse::Ok().json(items),
        Ok(_) => HttpResponse::NotFound().json("No orders found"),
        Err(error) => {
            error!("{:?}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Get handler for querying latest [Order] item for a specified menu item [Order::menu_item_id] for a specified table number.
pub async fn get_order_from_menu_item_and_table(
    data: web::Data<PgSqlOrderRepository>,
    path: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (table_number, menu_item_id) = path.into_inner();
    let order = read_order_item_from_table(data.as_ref(), table_number, menu_item_id).await;
    match order {
        Ok(item) if item.is_some() => HttpResponse::Ok().json(item),
        Ok(_) => HttpResponse::NotFound().json("No order found."), // No row found
        Err(error) => {
            error!("{:?}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Delete handler for removing latest [Order] item for a specified menu item [Order::menu_item_id] for a specified table number.
pub async fn delete_menu_item_from_order(
    data: web::Data<PgSqlOrderRepository>,
    path: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (table_number, menu_item_id) = path.into_inner();
    let delete_result =
        delete_order_item_from_table(data.as_ref(), menu_item_id, table_number).await;

    match delete_result {
        Ok(rows_deleted) if rows_deleted > 0 => HttpResponse::Ok().json("Order deleted."),
        Ok(_) => HttpResponse::NotFound().json("No orders found to delete."), // No rows found to delete
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
