// Usually I don't like using any Object-Relational Mapping, it feels like adding magic to the code.
// Some writing sql files is very common practice for me.

pub const CHECK_PASSWORD_QUERY: &str = r#"SELECT
        u.id,
        u.username,
        r.description as role
    FROM users u
    JOIN roles r ON u.role_id = r.id
    WHERE u.username = $1 AND u.password = crypt($2, u.password);
"#;

// As I see, this query will be used to render a menu on waiters app, to show how many orders are there
// And what waiter is in charge of which table and its status
pub const GET_ORDERS_QUERY: &str = r#"SELECT
    o.id,
    u.username,
    o.table_number,
    o.status,
    o.created_at
FROM orders o
JOIN users u ON o.waiter_assigned_id = u.id
ORDER BY o.created_at DESC;"#;

pub const UPDATE_ORDER_STATUS_QUERY: &str =
    r#"UPDATE orders SET status = $1::order_status WHERE id = $2"#;

pub const INSERT_ORDER_QUERY: &str = r#"INSERT INTO orders (table_number, waiter_assigned_id, status)
       VALUES ($1, $2, 'Created'::order_status) RETURNING id"#;

pub const INSERT_ORDER_ITEMS_QUERY: &str =
    r#"INSERT INTO order_items (order_id, menu_position_id)"#;

pub const DELETE_ORDER_QUERY: &str = r#"DELETE FROM orders where id = $1"#;

pub const GET_ORDER_INFO_QUERY: &str = r#"SELECT
    oi.id,
    o.status,
    o.created_at,
    m.time_to_cook,
    m.dish_title,
    m.cost
FROM orders o
JOIN order_items oi ON o.id = oi.order_id
JOIN menu m ON oi.menu_position_id = m.id
WHERE o.id = $1
ORDER BY o.created_at DESC;"#;

pub const DELETE_ORDER_ITEM_QUERY: &str = r#"DELETE FROM order_item WHERE id = $1"#;

pub const GET_MENU_QUERY: &str = r#"SELECT
    id,
    dish_title,
    cost,
    time_to_cook,
    is_available
FROM menu
ORDER BY dish_title;"#;

pub const UPDATE_MENU_QUERY: &str = r#"UPDATE menu
SET
    cost = $1,
    time_to_cook = $2,
    is_available = $3
WHERE id = $4;"#;

pub const DELETE_MENU_QUERY: &str = r#"DELETE FROM menu WHERE id = $1"#;

pub const INSERT_MENU_QUERY: &str =
    r#"INSERT INTO menu (dish_title, cost, time_to_cook) VALUES ($1, $2, $3)"#;
