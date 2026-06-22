use log::info;
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, CONNECTION, CONTENT_TYPE};
use serde_json::json;

pub async fn order_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://localhost:8080";
    let client = Client::new();
    let auth_header = "Basic YWRtaW46YWRtaW4xMjM=";

    let get_order_response = client
        .get(format!("{}/v1/menu", base_url))
        .header(AUTHORIZATION, auth_header)
        .send()
        .await;

    println!("{:#?}", get_order_response);

    let create_order_body = json!({
        "table_number": 1,
        "menu_item_ids": [5, 7, 9]
    });

    let create_response = client
        .post(format!("{}/v1/orders", base_url))
        .header(AUTHORIZATION, auth_header)
        .header(CONTENT_TYPE, "application/json")
        .header(CONNECTION, "keep-alive")
        .json(&create_order_body)
        .send()
        .await;

    info!("Order create response: {:?}", create_response);
    let r = create_response.unwrap();

    let order_response_json: serde_json::Value = r.json().await?;
    let order_id = order_response_json["id"].as_i64().unwrap() as i32;

    let add_items_body = json!({
        "menu_position_ids": [8, 4, 1]
    });

    client
        .post(format!("{}/v1/orders/{}", base_url, order_id))
        .header("Authorization", auth_header)
        .json(&add_items_body)
        .send()
        .await?;

    let statuses = vec!["Cooking", "Ready", "Served", "Paid"];
    for status in &statuses {
        let status_body = json!({
            "status": status
        });

        client
            .patch(format!("{}/v1/orders/{}", base_url, order_id))
            .header("Authorization", auth_header)
            .json(&status_body)
            .send()
            .await?;
    }

    client
        .get(format!("{}/v1/orders", base_url))
        .header("Authorization", auth_header)
        .send()
        .await?;

    let order_items_response = client
        .get(format!("{}/v1/orders/{}", base_url, order_id))
        .header("Authorization", auth_header)
        .send()
        .await?;

    let order_items: serde_json::Value = order_items_response.json().await?;
    println!("Order items: {}", serde_json::to_string_pretty(&order_items)?);

    Ok(())
}