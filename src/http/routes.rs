use crate::db::sql_connector;
use crate::model::error::{AppError, DatabaseError, HttpError};
use crate::model::model::{
    AppState, AuthenticatedUser, HttpAddOrderItemRequest, HttpCreateMenuItemRequest,
    HttpCreateMenuItemResponse, HttpCreateOrderRequest, HttpCreateOrderResponse,
    HttpDeleteOrderResponse, HttpGetMenuResponse, HttpGetOrderItemsResponse, HttpGetOrdersResponse,
    HttpUpdateMenuItemRequest, HttpUpdateOrderStatusRequest, MenuItem, Order, OrderInfo,
    OrderStatus, Role,
};
use crate::prometheus::prom::METRICS;
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, patch, post, HttpRequest, HttpResponse, Responder};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use prometheus::{Encoder, TextEncoder};
use sqlx::PgPool;
// ------------------------------------ Menu api ------------------------------------------

#[get("/v1/menu")]
pub async fn get_menu(req: HttpRequest, data: Data<AppState>) -> Result<impl Responder, AppError> {
    observe_duration("GET", "/v1/menu", async move || {
        auth_any(req, &data.pool).await?;
        let menu: Vec<MenuItem> = sql_connector::get_menu(&data.pool).await?;
        let response = HttpGetMenuResponse { menu };
        Ok(HttpResponse::Ok().json(response))
    })
    .await
}

#[post("/v1/menu")]
pub async fn post_new_menu_item(
    req: HttpRequest,
    payload: Json<HttpCreateMenuItemRequest>,
    data: Data<AppState>,
) -> Result<impl Responder, AppError> {
    observe_duration("POST", "/v1/menu", async move || {
        let (_, is_cook) = auth_cook(req, &data.pool).await?;
        if is_cook {
            let body: HttpCreateMenuItemRequest = payload.into_inner();
            let id = sql_connector::insert_menu_item(
                &data.pool,
                &body.dish_title,
                body.cost,
                body.time_to_cook,
            )
            .await?;
            let response = HttpCreateMenuItemResponse { id };
            Ok(HttpResponse::Created().json(response))
        } else {
            Err(AppError::Http(HttpError::NoPermissionError(
                "User is not authorized for this action".to_string(),
            )))?
        }
    })
    .await
}

#[patch("/v1/menu/{item_id}")]
pub async fn patch_menu_item(
    req: HttpRequest,
    path: Path<i32>,
    payload: Json<HttpUpdateMenuItemRequest>,
    data: Data<AppState>,
) -> Result<impl Responder, AppError> {
    observe_duration("PATCH", "/v1/menu/#", async move || {
        let (_, is_cook) = auth_cook(req, &data.pool).await?;

        if is_cook {
            let body = payload.into_inner();

            sql_connector::update_menu_item(
                &data.pool,
                body.cost,
                body.time_to_cook,
                body.is_available,
                path.into_inner(),
            )
            .await?;

            Ok(HttpResponse::Ok().finish())
        } else {
            Err(AppError::Http(HttpError::NoPermissionError(
                "Only administrators can update menu items".to_string(),
            )))?
        }
    })
    .await
}

#[delete("/v1/menu/{item_id}")]
pub async fn delete_menu_item(
    req: HttpRequest,
    path: Path<i32>,
    data: Data<AppState>,
) -> Result<impl Responder, AppError> {
    observe_duration("DELETE", "/v1/menu/#", async move || {
        let (_, is_cook) = auth_cook(req, &data.pool).await?;

        if is_cook {
            sql_connector::delete_menu_item(&data.pool, path.into_inner()).await?;
            Ok(HttpResponse::Ok().finish())
        } else {
            Err(AppError::Http(HttpError::NoPermissionError(
                "User is not authorized for this action".to_string(),
            )))?
        }
    })
    .await
}

// ------------------------------------ Order api ------------------------------------------
#[get("/v1/orders")]
pub async fn get_orders(
    req: HttpRequest,
    data: Data<AppState>,
) -> Result<impl Responder, AppError> {
    observe_duration("GET", "/v1/orders", async move || {
        auth_any(req, &data.pool).await?;
        let orders: Vec<Order> = sql_connector::get_orders(&data.pool).await?;
        let response = HttpGetOrdersResponse { orders };
        Ok(HttpResponse::Ok().json(response))
    })
    .await
}

#[get("/v1/orders/{order_id}")]
pub async fn get_order_items(
    order_id: Path<i32>,
    req: HttpRequest,
    data: Data<AppState>,
) -> Result<impl Responder, AppError> {
    observe_duration("GET", "/v1/orders/#", async move || {
        auth_any(req, &data.pool).await?;
        let order_items: Vec<OrderInfo> =
            sql_connector::get_order_info(&data.pool, order_id.into_inner()).await?;
        let response = HttpGetOrderItemsResponse { order_items };
        Ok(HttpResponse::Ok().json(response))
    })
    .await
}
// this api exists only to satisfy spec requirements
// From my perspective of data flow, order as entity should be enough.
#[get("/v1/orders/by_table/{table_id}")]
pub async fn get_order_items_by_table(
    table_id: Path<i32>,
    req: HttpRequest,
    data: Data<AppState>,
) -> Result<impl Responder, AppError> {
    observe_duration("GET", "/v1/orders/by_table/#", async move || {
        auth_any(req, &data.pool).await?;
        let order_items: Vec<OrderInfo> =
            sql_connector::get_order_items_by_table(&data.pool, table_id.into_inner()).await?;
        let response = HttpGetOrderItemsResponse { order_items };
        Ok(HttpResponse::Ok().json(response))
    })
        .await
}


#[post("/v1/orders")]
pub async fn post_order(
    req: HttpRequest,
    payload: Json<HttpCreateOrderRequest>,
    data: Data<AppState>,
) -> Result<impl Responder, AppError> {
    observe_duration("POST", "/v1/orders", async move || {
        let (id, is_waiter) = auth_waiter(req, &data.pool).await?;
        if is_waiter {
            let body: HttpCreateOrderRequest = payload.into_inner();
            let id =
                sql_connector::create_order(&data.pool, body.table_number, id, body.menu_item_ids)
                    .await?;
            let response = HttpCreateOrderResponse { id };
            Ok(HttpResponse::Created().json(response))
        } else {
            Err(AppError::Http(HttpError::NoPermissionError(
                "User is not authorized for this action".to_string(),
            )))?
        }
    })
    .await
}

#[post("/v1/orders/{order_id}")]
pub async fn add_order_items(
    req: HttpRequest,
    path: Path<i32>,
    payload: Json<HttpAddOrderItemRequest>,
    data: Data<AppState>,
) -> Result<impl Responder, AppError> {
    observe_duration("POST", "/v1/orders/#", async move || {
        let (_, is_waiter) = auth_waiter(req, &data.pool).await?;

        if is_waiter {
            let order_id = path.into_inner();
            let body = payload.into_inner();

            sql_connector::insert_order_items(&data.pool, order_id, &body.menu_position_ids)
                .await?;

            Ok(HttpResponse::Created().finish())
        } else {
            Err(AppError::Http(HttpError::NoPermissionError(
                "User is not authorized for this action".to_string(),
            )))?
        }
    })
    .await
}

#[delete("/v1/orders/items/{order_item_id}")]
pub async fn delete_order_item(
    req: HttpRequest,
    path: Path<i32>,
    data: Data<AppState>,
) -> Result<impl Responder, AppError> {
    observe_duration("DELETE", "/v1/orders/items/#", async move || {
        let (_, is_waiter) = auth_waiter(req, &data.pool).await?;

        if is_waiter {
            sql_connector::delete_order_item(&data.pool, path.into_inner()).await?;
            Ok(HttpResponse::Ok().finish())
        } else {
            Err(AppError::Http(HttpError::NoPermissionError(
                "User is not authorized for this action".to_string(),
            )))?
        }
    })
    .await
}

#[delete("/v1/orders/{order_id}")]
pub async fn delete_order(
    req: HttpRequest,
    path: Path<i32>,
    data: Data<AppState>,
) -> Result<impl Responder, AppError> {
    observe_duration("DELETE", "/v1/orders/#", async move || {
        let (_, is_admin) = auth_admin(req, &data.pool).await?;

        if is_admin {
            let rows_affected =
                sql_connector::delete_order(&data.pool, path.into_inner()).await? as i32;
            let response = HttpDeleteOrderResponse {
                entries_deleted: rows_affected,
            };
            Ok(HttpResponse::Ok().json(response))
        } else {
            Err(AppError::Http(HttpError::NoPermissionError(
                "User is not authorized for this action".to_string(),
            )))?
        }
    })
    .await
}

#[patch("/v1/orders/{order_id}")]
pub async fn update_order_status(
    req: HttpRequest,
    path: Path<i32>,
    payload: Json<HttpUpdateOrderStatusRequest>,
    data: Data<AppState>,
) -> Result<impl Responder, AppError> {
    observe_duration("PATCH", "/v1/orders/#", async move || {
        let order_id = path.into_inner();
        let body: HttpUpdateOrderStatusRequest = payload.into_inner();

        match body.status {
            OrderStatus::Cooking | OrderStatus::Ready => {
                let (_, is_cook) = auth_cook(req, &data.pool).await?;
                if !is_cook {
                    return Err(AppError::Http(HttpError::NoPermissionError(
                        "Only cooks can update cooking status".to_string(),
                    )))?;
                }
            }
            OrderStatus::Created | OrderStatus::Paid | OrderStatus::Served => {
                let (_, is_waiter) = auth_waiter(req, &data.pool).await?;
                if !is_waiter {
                    return Err(AppError::Http(HttpError::NoPermissionError(
                        "Only waiters can mark orders as served".to_string(),
                    )))?;
                }
            }
            OrderStatus::Cancelled => {
                let (_, is_admin) = auth_admin(req, &data.pool).await?;
                if !is_admin {
                    return Err(AppError::Http(HttpError::NoPermissionError(
                        "Only admins can cancel or mark orders as paid".to_string(),
                    )))?;
                }
            }
        }

        let rows_affected =
            sql_connector::update_order_status(&data.pool, order_id, body.status).await?;

        if rows_affected != 0 {
            Ok(HttpResponse::Ok().finish())
        } else {
            Err(AppError::Database(DatabaseError::ValueNotFoundError(
                format!("Order not found: {}", order_id),
            )))?
        }
    })
    .await
}

// --------------------------------------- Utility ---------------------------------------------

#[get("/metrics")]
async fn metrics() -> HttpResponse {
    let encoder = TextEncoder::new();
    let registry = &METRICS.lock().await.registry;
    let metric_families = registry.gather();

    let mut buffer = Vec::new();
    match encoder.encode(&metric_families, &mut buffer) {
        Ok(_) => HttpResponse::Ok()
            .content_type("text/plain; version=0.0.4; charset=utf-8")
            .body(buffer),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to encode metrics: {e}"))
        }
    }
}

#[get("/health")]
async fn health_check(_: Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn observe_duration(
    method: &str,
    path: &str,
    func: impl AsyncFnOnce() -> Result<HttpResponse, AppError>,
) -> Result<impl Responder, AppError> {
    let timer = METRICS
        .lock()
        .await
        .http_request_duration
        .with_label_values(&[method, path])
        .start_timer();
    let response = func().await;
    timer.observe_duration();
    response
}

// ----------------------------------------- Auth ------------------------------------------
// It would be 5 times better to implement this part as Auth middleware, very time consuming
async fn auth_any(req: HttpRequest, pool: &PgPool) -> Result<i32, AppError> {
    auth(req, pool).await.map(|user| user.id)
}

async fn auth_waiter(req: HttpRequest, pool: &PgPool) -> Result<(i32, bool), AppError> {
    auth(req, pool)
        .await
        .map(|user| (user.id, is_admin(&user) || is_waiter(&user)))
}

async fn auth_cook(req: HttpRequest, pool: &PgPool) -> Result<(i32, bool), AppError> {
    auth(req, pool)
        .await
        .map(|user| (user.id, is_admin(&user) || is_cook(&user)))
}

async fn auth_admin(req: HttpRequest, pool: &PgPool) -> Result<(i32, bool), AppError> {
    auth(req, pool).await.map(|user| (user.id, is_admin(&user)))
}

async fn auth(req: HttpRequest, pool: &PgPool) -> Result<AuthenticatedUser, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| Some(v.to_str()))
        .and_then(|auth_header| auth_header.ok())
        .and_then(|auth_header| auth_header.split_once(" "));

    match auth_header {
        Some((auth_method, auth_value)) => {
            if auth_method == "Basic" {
                let decoded_creds = BASE64_STANDARD
                    .decode(auth_value)
                    .ok()
                    .and_then(|u8| String::from_utf8(u8).ok())
                    .and_then(|creds| {
                        creds
                            .split_once(':')
                            .map(|(u, p)| (u.to_string(), p.to_string()))
                    });

                match decoded_creds {
                    Some((username, password)) => {
                        Ok(
                            sql_connector::check_user(pool, username.as_str(), password.as_str())
                                .await?,
                        )
                    }
                    None => Err(HttpError::UserDecodeError)?,
                }
            } else {
                Err(HttpError::WrongAuthMethodError(auth_method.to_string()))?
            }
        }
        None => Err(HttpError::NoAuthHeaderError)?,
    }
}

fn is_admin(user: &AuthenticatedUser) -> bool {
    match user.role {
        Role::Admin => true,
        _ => false,
    }
}

fn is_cook(user: &AuthenticatedUser) -> bool {
    match user.role {
        Role::Cook => true,
        _ => false,
    }
}

fn is_waiter(user: &AuthenticatedUser) -> bool {
    match user.role {
        Role::Waiter => true,
        _ => false,
    }
}
