use crate::model::model::OrderStatus;
use crate::test::test_data::{
    AUTH_HEADER_ADMIN, AUTH_HEADER_COOK, AUTH_HEADER_WAITER, DEFAULT_MENU_RESPONSE,
    UPDATED_MENU_RESPONSE, add_order_items_body, create_change_status_body, create_menu_item_body,
    create_order_body, patch_menu_item_body,
};
use reqwest::header::AUTHORIZATION;
use reqwest::{Client, Response, StatusCode};
use std::time::Duration;
use testcontainers::compose::DockerCompose;

// here is not a documentation, but my thoughts as I was designing this app.

// In this test I would like to show the intended workflow.
// We have 2 sides working, the kitchen, and waiters.

// 2 Applications.

// As I see the use, some couple walks into the restaurant, sits at the table.

// Waiter comes, and first thing that presented is a menu.
// Is the item available, how much does it cost, how long will it be cooked.
// So

// 1) Application make a request to get current menu items
// 2) After order can be created.
// 3) Cooks take a look at the order, for that they need to get the order items from order.
// 4) Cooks receive an order and move it to status "Cooking" when ready
// 5) Meanwhile customers decided to order something else. so new items are added to the order.
// 6) When cooking is done, the order is moved to status "Ready"
// 7) When the order is ready, it is served and the order is moved to status "Served"
// 8) If the customer wants to add something else, new items are added and the order moves to status "Cooking
// 9) Finally, the order is moved to status "Paid"

// At any moment order can be canceled, but only by admin.

// I have decided not to delete orders after they have been served to preserve history.
// But it is possible to do so by someone with admin rights.
#[tokio::test]
async fn intended_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let mut compose = DockerCompose::with_local_client(&["./docker-compose.yaml"]).with_wait(false);
    compose.up().await?;

    let app_container = compose.service("app").expect("app not running");
    let host = app_container.get_host().await?;
    let port: u16 = app_container.get_host_port_ipv4(8080).await?;

    let base_url: String = format!("http://{}:{}", host, port);
    let client: Client = Client::new();

    let is_healthy: bool = wait_for_service(&base_url, &client).await;
    assert!(is_healthy);

    let menu_response: Response = client
        .get(format!("{}/v1/menu", base_url))
        .header(AUTHORIZATION, AUTH_HEADER_WAITER)
        .send()
        .await?;

    assert!(menu_response.status().is_success());
    assert_eq!(menu_response.text().await?, DEFAULT_MENU_RESPONSE);

    let create_response: Response = client
        .post(format!("{}/v1/orders", base_url))
        .header(AUTHORIZATION, AUTH_HEADER_WAITER)
        .json(&create_order_body())
        .send()
        .await?;

    assert!(create_response.status().is_success());
    let order_response_json: serde_json::Value = create_response.json().await?;
    let order_id: i32 = order_response_json["id"].as_i64().unwrap() as i32;

    let order_items_response = client
        .get(format!("{}/v1/orders/{}", base_url, order_id))
        .header("Authorization", AUTH_HEADER_COOK)
        .send()
        .await?;

    assert!(order_items_response.status().is_success());

    let change_status_response: Response = client
        .patch(format!("{}/v1/orders/{}", base_url, order_id))
        .header("Authorization", AUTH_HEADER_COOK)
        .json(&create_change_status_body(OrderStatus::Cooking))
        .send()
        .await?;

    assert!(change_status_response.status().is_success());

    let add_items_response = client
        .post(format!("{}/v1/orders/{}", base_url, order_id))
        .header("Authorization", AUTH_HEADER_WAITER)
        .json(&add_order_items_body())
        .send()
        .await?;

    assert!(add_items_response.status().is_success());

    let change_status_response: Response = client
        .patch(format!("{}/v1/orders/{}", base_url, order_id))
        .header("Authorization", AUTH_HEADER_COOK)
        .json(&create_change_status_body(OrderStatus::Ready))
        .send()
        .await?;

    assert!(change_status_response.status().is_success());

    let change_status_response: Response = client
        .patch(format!("{}/v1/orders/{}", base_url, order_id))
        .header("Authorization", AUTH_HEADER_WAITER)
        .json(&create_change_status_body(OrderStatus::Served))
        .send()
        .await?;

    assert!(change_status_response.status().is_success());

    // sketchy waiter wants to cancel order and keep the money, can't let that happen.
    let change_status_response: Response = client
        .patch(format!("{}/v1/orders/{}", base_url, order_id))
        .header("Authorization", AUTH_HEADER_WAITER)
        .json(&create_change_status_body(OrderStatus::Cancelled))
        .send()
        .await?;

    assert_eq!(change_status_response.status(), StatusCode::FORBIDDEN);

    // waiter decides to play by the rules and accept payment.
    let change_status_response: Response = client
        .patch(format!("{}/v1/orders/{}", base_url, order_id))
        .header("Authorization", AUTH_HEADER_WAITER)
        .json(&create_change_status_body(OrderStatus::Paid))
        .send()
        .await?;

    assert!(change_status_response.status().is_success());

    let remove_order = client
        .delete(format!("{}/v1/orders/{}", base_url, order_id))
        .header("Authorization", AUTH_HEADER_ADMIN)
        .send()
        .await?;

    assert!(remove_order.status().is_success());
    // let remove_order_json: serde_json::Value = remove_order.json().await?;
    // let order_id: i32 = remove_order_json["entries_deleted"].as_i64().unwrap() as i32;
    // assert_eq!(order_id, 1);
    Ok(())
}

// In this test let's emulate cooks' behavior
// A lot of stuff needs to be added to the menu
// Some positions are outdated, some are not available.
// Let's check if menu api works as expected.
#[tokio::test]
async fn cooks_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let mut compose = DockerCompose::with_local_client(&["./docker-compose.yaml"]).with_wait(false);
    compose.up().await?;

    let app_container = compose.service("app").expect("app not running");
    let host = app_container.get_host().await?;
    let port: u16 = app_container.get_host_port_ipv4(8080).await?;

    let base_url: String = format!("http://{}:{}", host, port);
    let client: Client = Client::new();

    let is_healthy: bool = wait_for_service(&base_url, &client).await;
    assert!(is_healthy);

    let menu_response: Response = client
        .get(format!("{}/v1/menu", base_url))
        .header(AUTHORIZATION, AUTH_HEADER_COOK)
        .send()
        .await?;

    assert!(menu_response.status().is_success());
    assert_eq!(menu_response.text().await?, DEFAULT_MENU_RESPONSE);

    let menu_item_response = client
        .post(format!("{}/v1/menu", base_url))
        .header("Authorization", AUTH_HEADER_COOK)
        .json(&create_menu_item_body())
        .send()
        .await?;

    assert!(menu_item_response.status().is_success());
    let menu_item_json: serde_json::Value = menu_item_response.json().await?;
    let item_id: i32 = menu_item_json["id"].as_i64().unwrap() as i32;

    let patched_menu_item_response = client
        .patch(format!("{}/v1/menu/{}", base_url, item_id))
        .header("Authorization", AUTH_HEADER_COOK)
        .json(&patch_menu_item_body())
        .send()
        .await?;

    assert!(patched_menu_item_response.status().is_success());
    println!("{}", patched_menu_item_response.text().await?);

    let menu_response = client
        .get(format!("{}/v1/menu", base_url))
        .header(AUTHORIZATION, AUTH_HEADER_COOK)
        .send()
        .await?;

    assert!(menu_response.status().is_success());
    assert_eq!(menu_response.text().await?, UPDATED_MENU_RESPONSE);

    let deleted_menu_item_response = client
        .delete(format!("{}/v1/menu/{}", base_url, item_id))
        .header("Authorization", AUTH_HEADER_COOK)
        .send()
        .await?;

    assert!(deleted_menu_item_response.status().is_success());

    let menu_response = client
        .get(format!("{}/v1/menu", base_url))
        .header(AUTHORIZATION, AUTH_HEADER_COOK)
        .send()
        .await?;

    assert!(menu_response.status().is_success());
    assert_eq!(menu_response.text().await?, DEFAULT_MENU_RESPONSE);

    Ok(())
}

async fn wait_for_service(host: &String, client: &Client) -> bool {
    let mut attempts = 0;
    let max_attempts = 20;

    let health_url = format!("{}/health", host);
    loop {
        attempts += 1;
        if attempts > max_attempts {
            break false;
        }
        match client.get(&health_url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    break true;
                }
            }
            Err(_) => {}
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
