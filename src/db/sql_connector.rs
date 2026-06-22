use crate::config::PostgresConfig;
use crate::db::sql;
use crate::model::error::DatabaseError;

use crate::db::sql::{CHECK_PASSWORD_QUERY, DELETE_MENU_QUERY, DELETE_ORDER_ITEM_QUERY, DELETE_ORDER_QUERY, GET_MENU_QUERY, GET_ORDER_INFO_QUERY, GET_ORDERS_QUERY, INSERT_MENU_QUERY, UPDATE_MENU_QUERY, UPDATE_ORDER_STATUS_QUERY, SELECT_ORDER_FROM_TABLE};
use crate::model::model::{AuthenticatedUser, MenuItem, Order, OrderInfo, OrderStatus};
use crate::prometheus::prom::METRICS;
use sqlx::postgres::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Postgres, QueryBuilder};

pub async fn create_pool(postgres_config: &PostgresConfig) -> Result<PgPool, DatabaseError> {
    let connect_opt = PgConnectOptions::new()
        .host(&postgres_config.host)
        .port(postgres_config.port)
        .database(&postgres_config.data_base)
        .username(&postgres_config.user)
        .password(&postgres_config.pwd);

    PgPoolOptions::new()
        .max_connections(postgres_config.pool_size)
        .connect_with(connect_opt)
        .await
        .map_err(|err| DatabaseError::PoolCreationError(err.to_string()))
}

pub async fn check_user(
    pool: &PgPool,
    user_login: &str,
    user_password: &str,
) -> Result<AuthenticatedUser, DatabaseError> {
    observe("check_user", || async {
        sqlx::query_as::<_, AuthenticatedUser>(CHECK_PASSWORD_QUERY)
            .bind(user_login)
            .bind(user_password)
            .fetch_optional(pool)
            .await
            .map_err(|err| DatabaseError::BadQueryError(err.to_string()))
            .and_then(|opt| {
                opt.ok_or(DatabaseError::ValueNotFoundError(format!(
                    "User {} not found",
                    user_login
                )))
            })
    })
    .await
}

pub async fn insert_order_items(
    pool: &PgPool,
    order_id: i32,
    menu_item_ids: &[i32],
) -> Result<u64, DatabaseError> {
    observe("insert_order_items", || async {
        if !menu_item_ids.is_empty() {
            let mut builder: QueryBuilder<Postgres> =
                QueryBuilder::new(sql::INSERT_ORDER_ITEMS_QUERY);

            builder
                .push(" ")
                .push_values(menu_item_ids.iter(), |mut b, menu_id| {
                    b.push_bind(order_id).push_bind(menu_id);
                });

            builder
                .build()
                .execute(pool)
                .await
                .map(|result| result.rows_affected())
                .map_err(|err| DatabaseError::BadQueryError(err.to_string()))
        } else {
            Ok(0)
        }
    })
    .await
}

pub async fn create_order(
    pool: &PgPool,
    table_number: i32,
    waiter_id: i32,
    menu_item_ids: Vec<i32>,
) -> Result<i32, DatabaseError> {
    observe("create_order", || async {
        let mut tx = pool
            .begin()
            .await
            .map_err(|err| DatabaseError::TransactionCreationError(err.to_string()))?;

        let id: i32 = sqlx::query_scalar(sql::INSERT_ORDER_QUERY)
            .bind(table_number)
            .bind(waiter_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|err| DatabaseError::BadQueryError(err.to_string()))?;

        let mut builder = QueryBuilder::new(sql::INSERT_ORDER_ITEMS_QUERY);
        builder
            .push(" ")
            .push_values(menu_item_ids.iter(), |mut b, menu_id| {
                b.push_bind(id).push_bind(menu_id);
            });

        builder
            .build()
            .execute(&mut *tx)
            .await
            .map_err(|e| DatabaseError::BadQueryError(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|err| DatabaseError::TransactionCreationError(err.to_string()))?;

        Ok(id)
    })
    .await
}

pub async fn get_orders(pool: &PgPool) -> Result<Vec<Order>, DatabaseError> {
    observe("get_orders", || async {
        sqlx::query_as::<_, Order>(GET_ORDERS_QUERY)
            .fetch_all(pool)
            .await
            .map_err(|err| DatabaseError::BadQueryError(err.to_string()))
    })
    .await
}

pub async fn update_order_status(
    pool: &PgPool,
    order_id: i32,
    status: OrderStatus,
) -> Result<u64, DatabaseError> {
    observe("update_order_status", || async {
        sqlx::query(UPDATE_ORDER_STATUS_QUERY)
            .bind(status)
            .bind(order_id)
            .execute(pool)
            .await
            .map(|result| result.rows_affected())
            .map_err(|err| DatabaseError::BadQueryError(err.to_string()))
    })
    .await
}

pub async fn delete_order(pool: &PgPool, order_id: i32) -> Result<u64, DatabaseError> {
    observe("delete_order", || async {
        sqlx::query(DELETE_ORDER_QUERY)
            .bind(order_id)
            .execute(pool)
            .await
            .map(|result| result.rows_affected())
            .map_err(|err| DatabaseError::BadQueryError(err.to_string()))
    })
    .await
}

pub async fn get_order_info(pool: &PgPool, order_id: i32) -> Result<Vec<OrderInfo>, DatabaseError> {
    observe("get_order_info", || async {
        sqlx::query_as::<_, OrderInfo>(GET_ORDER_INFO_QUERY)
            .bind(order_id)
            .fetch_all(pool)
            .await
            .map_err(|err| DatabaseError::BadQueryError(err.to_string()))
    })
    .await
}

pub async fn delete_order_item(pool: &PgPool, order_item_id: i32) -> Result<u64, DatabaseError> {
    observe("delete_order_item", || async {
        sqlx::query(DELETE_ORDER_ITEM_QUERY)
            .bind(order_item_id)
            .execute(pool)
            .await
            .map(|result| result.rows_affected())
            .map_err(|err| DatabaseError::BadQueryError(err.to_string()))
    })
    .await
}

pub async fn get_menu(pool: &PgPool) -> Result<Vec<MenuItem>, DatabaseError> {
    observe("get_menu", || async {
        sqlx::query_as::<_, MenuItem>(GET_MENU_QUERY)
            .fetch_all(pool)
            .await
            .map_err(|err| DatabaseError::BadQueryError(err.to_string()))
    })
    .await
}

pub async fn update_menu_item(
    pool: &PgPool,
    cost: i64,
    time_to_cook: i32,
    is_available: bool,
    item_id: i32,
) -> Result<u64, DatabaseError> {
    observe("update_menu_item", || async {
        sqlx::query(UPDATE_MENU_QUERY)
            .bind(cost)
            .bind(time_to_cook)
            .bind(is_available)
            .bind(item_id)
            .execute(pool)
            .await
            .map(|result| result.rows_affected())
            .map_err(|err| DatabaseError::BadQueryError(err.to_string()))
    })
    .await
}

pub async fn delete_menu_item(pool: &PgPool, item_id: i32) -> Result<u64, DatabaseError> {
    observe("delete_menu_item", || async {
        sqlx::query(DELETE_MENU_QUERY)
            .bind(item_id)
            .execute(pool)
            .await
            .map(|result| result.rows_affected())
            .map_err(|err| DatabaseError::BadQueryError(err.to_string()))
    })
    .await
}

pub async fn get_order_items_by_table(pool: &PgPool, table_number: i32) -> Result<Vec<OrderInfo>, DatabaseError> {
    observe("get_order_info_from_table", || async {
        sqlx::query_as::<_, OrderInfo>(SELECT_ORDER_FROM_TABLE)
            .bind(table_number)
            .fetch_all(pool)
            .await
            .map_err(|err| DatabaseError::BadQueryError(err.to_string()))
    })
        .await
}

pub async fn insert_menu_item(
    pool: &PgPool,
    dish_title: &str,
    cost: i64,
    time_to_cook: i32,
) -> Result<i32, DatabaseError> {
    observe("insert_menu_item", || async {
        sqlx::query_scalar::<_, i32>(INSERT_MENU_QUERY)
            .bind(dish_title)
            .bind(cost)
            .bind(time_to_cook)
            .fetch_one(pool)
            .await
            .map_err(|err| DatabaseError::BadQueryError(err.to_string()))
    })
    .await
}

async fn observe<T>(
    operation: &str,
    func: impl AsyncFnOnce() -> Result<T, DatabaseError>,
) -> Result<T, DatabaseError> {
    let metrics = METRICS.lock().await;
    let timer = metrics
        .db_request_duration
        .with_label_values(&[operation])
        .start_timer();
    let response = func().await;
    timer.observe_duration();
    response
}
