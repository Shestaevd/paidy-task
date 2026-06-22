use crate::config::AppConfig;
use crate::db::sql_connector;
use crate::model::model::AppState;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use log::{error, info, LevelFilter};
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use crate::test::infinite_test::order_lifecycle;

mod config;
mod db;
mod http;
mod model;
mod test;

#[actix_web::main]
async fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Debug)
        .init();

    let conf_path: String = env::var("CONF").unwrap_or_else(|_| "src/resources".to_string());

    let conf_env_path: String = env::var("ENV").unwrap_or_else(|_| "base".to_string());

    let config: AppConfig = config::get_config(conf_path.as_str(), conf_env_path.as_str(), "APP")
        .unwrap_or_else(|e| {
            error!("Failed to load config: {e}");
            std::process::exit(1);
        });

    let host: String = config.http.host.clone();
    let port: u16 = config.http.port;

    let pool: PgPool = sql_connector::create_pool(&config.postgres)
        .await
        .unwrap_or_else(|e| {
            error!("Failed to create database pool: {e}");
            std::process::exit(1);
        });

    info!("starting");

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;

        if let Err(e) = order_lifecycle().await {
            eprintln!("Order lifecycle test failed: {}", e);
        } else {
            println!("Order lifecycle test completed successfully!");
        }
    });
    
    HttpServer::new(move || {
        let cors: Cors = Cors::default()
            // should be in scope of prod network, so only other services could have access.
            // But for this task I will leave it like this
            .allow_any_origin()
            .allow_any_header() // should also spend more time, figuring out allowed headers.
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(AppState {
                pool: pool.clone(),
            }))
            .service(http::routes::get_menu)
            .service(http::routes::patch_menu_item)
            .service(http::routes::delete_menu_item)
            .service(http::routes::post_new_menu_item)
            .service(http::routes::post_order)
            .service(http::routes::delete_order)
            .service(http::routes::update_order_status)
            .service(http::routes::add_order_items)
            .service(http::routes::delete_order_item)
            .service(http::routes::get_orders)
            .service(http::routes::get_order_items)
            .service(http::routes::health_check)
    })
    .bind(("127.0.0.1", port))
    .unwrap()
    .run()
    .await
    .expect("panic message")
}
