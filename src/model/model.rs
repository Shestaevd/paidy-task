use crate::model::error::{AppError, DatabaseError, HttpError};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgTypeInfo, PgValueRef};
use sqlx::{Decode, FromRow, PgPool, Postgres, Type};
use sqlx::postgres::types::PgInterval;

pub struct AppState {
    pub pool: PgPool
}

// Sql objects

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: i32,
    pub dish_title: String,
    pub cost: i64,
    pub time_to_cook: i32,
    pub is_available: bool,
}

#[derive(Debug)]
pub enum Role {
    Admin,
    Waiter,
    Cook
}

impl Type<Postgres> for Role {
    fn type_info() -> PgTypeInfo {
        <String as Type<Postgres>>::type_info()
    }
}

impl<'r> Decode<'r, Postgres> for Role {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as Decode<Postgres>>::decode(value)?;
        match s.as_str() {
            "Admin" => Ok(Role::Admin),
            "Waiter" => Ok(Role::Waiter),
            "Cook" => Ok(Role::Cook),
            _ => Err(format!("Invalid role: {}", s).into()),
        }
    }
}


#[derive(Debug, FromRow)]
pub struct AuthenticatedUser {
    pub id: i32,
    pub username: String,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Type)]
#[sqlx(type_name = "order_status")]
pub enum OrderStatus {
    Created,
    Cooking,
    Ready,
    Served,
    Paid,
    Cancelled
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub username: String,
    pub table_number: i32,
    pub status: OrderStatus,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct OrderInfo {
    pub id: i32,
    pub status: OrderStatus,
    pub created_at: NaiveDateTime,
    pub time_to_cook: i32,
    pub dish_title: String,
    pub cost: i64,
}

// Http requests

#[derive(Debug, Deserialize)]
pub struct HttpUpdateOrderStatusRequest {
    pub status: OrderStatus,
}

#[derive(Debug, Deserialize)]
pub struct HttpCreateOrderRequest {
    pub table_number: i32,
    pub menu_item_ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct HttpCreateMenuItemRequest {
    pub dish_title: String,
    pub cost: i64,
    pub time_to_cook: i32,
}

#[derive(Debug, Deserialize)]
pub struct HttpUpdateMenuItemRequest {
    pub cost: i64,
    pub time_to_cook: i32,
    pub is_available: bool,
}

#[derive(Debug, Deserialize)]
pub struct HttpAddOrderItemRequest {
    pub menu_position_ids: Vec<i32>,
}

// Http responses

#[derive(Debug, Serialize)]
pub struct HttpGetMenuResponse {
    pub menu: Vec<MenuItem>,
}

#[derive(Debug, Serialize)]
pub struct HttpCreateOrderResponse {
    pub id: i32,
}

#[derive(Debug, Serialize)]
pub struct HttpGetOrdersResponse {
    pub orders: Vec<Order>,
}

#[derive(Debug, Serialize)]
pub struct HttpGetOrderItemsResponse {
    pub order_items: Vec<OrderInfo>,
}

#[derive(Debug, Serialize)]
pub struct HttpErrorResponse {
    pub error: String,
    pub message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Http(HttpError::NoAuthHeaderError) => StatusCode::UNAUTHORIZED,
            AppError::Http(HttpError::NoSuchUserError(_)) => StatusCode::NOT_FOUND,
            AppError::Http(HttpError::NoPermissionError(_)) => StatusCode::FORBIDDEN,
            AppError::Http(HttpError::WrongAuthMethodError(_)) => StatusCode::BAD_REQUEST,
            AppError::Database(DatabaseError::ValueNotFoundError(_)) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let message = self.to_string();
        let payload = HttpErrorResponse {
            error: status.canonical_reason().unwrap_or("Unknown Error").to_string(),
            message,
        };

        HttpResponse::build(status).json(payload)
    }
}