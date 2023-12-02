#[cfg(test)]
mod clients_simulation {
    use actix_web::rt::time::sleep;
    use rand::Rng;
    use restaurant::presentation::OrderRequest;
    use std::{collections::HashMap, time::Duration};

    // Test parameters
    const RESTAURANT_TABLES_QTY: usize = 500;
    const CONCURRENT_CLIENTS_QTY: usize = 50;
    const TOTAL_REQS_PER_CLIENT: usize = 25;

    // Connection parameters
    const SERVER_ADDRS: &str = "http://127.0.0.1:8080"; // Address to spawn the test server into

    /// Integration test that spawns the HTTP REST API Server
    /// and simulates concurrent clients randomly executing requests.
    ///
    #[actix_web::test]
    async fn simulate_client_behavior() {
        let client_simulation = |_client_number| async {
            let client = reqwest::Client::new();
            let menu_item_id = rand::random::<usize>() % TOTAL_MENU_ITEMS_QTY;
            let table_number = rand::random::<usize>() % RESTAURANT_TABLES_QTY;
            let mut rng = rand::thread_rng();

            for _ in 1..=TOTAL_REQS_PER_CLIENT {
                // Randomly choose from server api request per client simulation
                match rng.gen_range(1..=4) {
                    1 => {
                        // simulate create
                        let response = client
                            .post(format!("{}/v1/orders", SERVER_ADDRS))
                            .json(&OrderRequest {
                                table_number: table_number as i32,
                                menu_item_id: menu_item_id as i32,
                            })
                            .send()
                            .await;
                        assert!(response.is_ok());
                    }
                    2 => {
                        // simulate query all items from table request
                        let response = client
                            .get(format!(
                                "{}/v1/tables/{}/orders",
                                SERVER_ADDRS, table_number
                            ))
                            .send()
                            .await;
                        assert!(response.is_ok());

                        // Delete request.
                        // After this querying all items, run 50% probability to execute 1 delete query by id
                        let orders_json_data = response
                            .unwrap()
                            .json::<Vec<HashMap<String, serde_json::Value>>>()
                            .await;
                        match orders_json_data {
                            Err(_) => continue, // If error, then "No orders found". Continue the loop.
                            _ => (),
                        }

                        for order_json in orders_json_data.unwrap() {
                            let order_id = order_json.get("order_id");
                            match order_id {
                                Some(id) => {
                                    let response = client
                                        .get(format!(
                                            "{}/v1/orders/{}",
                                            SERVER_ADDRS,
                                            id.to_string()
                                        ))
                                        .send()
                                        .await;
                                    assert!(response.is_ok());
                                    break; // finish this deletion test after deleting 1 order from the list
                                }
                                None => (),
                            }
                        }
                    }
                    3 => {
                        // simulate query specific item from table request
                        let response = client
                            .get(format!(
                                "{}/v1/tables/{}/menu_items/{}",
                                SERVER_ADDRS, table_number, menu_item_id
                            ))
                            .send()
                            .await;
                        assert!(response.is_ok());
                    }
                    4 => {
                        // simulate delete specific item from specific table request
                        let response = client
                            .delete(format!(
                                "{}/v1/tables/{}/menu_items/{}",
                                SERVER_ADDRS, table_number, menu_item_id
                            ))
                            .send()
                            .await;
                        assert!(response.is_ok());
                    }
                    _ => unreachable!(),
                }
                // Add a small delay between requests
                sleep(Duration::from_millis(100)).await;
            }
        };

        // Simulate multiple clients
        let client_tasks: Vec<_> = (1..=CONCURRENT_CLIENTS_QTY)
            .map(|i| client_simulation(i))
            .collect();

        // Run all client tasks concurrently
        let _ = futures::future::join_all(client_tasks).await;

        return ();
    }

    // Total registered items when setting up the TABLE 'menu_items'.
    const TOTAL_MENU_ITEMS_QTY: usize = 50; // Do not change.
}
