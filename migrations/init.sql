-- even though this assignment do not specifically require implementation of auth, I think it will not be
-- unnecessary to add users and roles, as part of local oauth mechanism, stored in the same database for simplification.
-- password will be cyphered as part of pgcrypto extension.

-- I also would love to add session mechanism, but it will require either sticky sessions or other service to act as auth,
-- and with what I have planned, this task will already consume good amount of my time. So I hope you can forgive me 😃.

-- usually when implementing roles, we use access type behavior, like READ, WRITE, DELETE. But for this project,
-- to make it simple I just used admin, waiter and cook.
-- Waiter can create order for specific table and get assigned to it.
-- Only that waiter can modify this order on the table he assigned to.
-- Cooks can modify menu
-- admin can do everything.

CREATE TABLE IF NOT EXISTS roles
(
    id          SERIAL PRIMARY KEY,
    description TEXT
);

CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS users
(
    id         SERIAL PRIMARY KEY,
    username   TEXT UNIQUE NOT NULL,
    password   TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    role_id    INTEGER            NOT NULL,
    CONSTRAINT fk_user_role FOREIGN KEY (role_id)
        REFERENCES roles (id)
        ON DELETE RESTRICT
);

CREATE TABLE IF NOT EXISTS menu
(
    id           SERIAL PRIMARY KEY,
    dish_title   TEXT UNIQUE,
    cost         BIGINT   NOT NULL,
    time_to_cook INT NOT NULL,
    is_available BOOLEAN DEFAULT TRUE
);

CREATE TYPE order_status AS ENUM (
    'Created',
    'Cooking',
    'Ready',
    'Served',
    'Paid',
    'Cancelled'
);

CREATE TABLE IF NOT EXISTS orders
(
    id                 SERIAL PRIMARY KEY,
    table_number       INTEGER      NOT NULL,
    waiter_assigned_id INTEGER      NOT NULL,
    status             order_status NOT NULL,
    created_at         TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_waiter_assigned FOREIGN KEY (waiter_assigned_id)
        REFERENCES users (id)
        ON DELETE RESTRICT
);

-- having an index on order_id and menu_position is a good idea, yea, for this assignment I decided to leave it as it is.
-- b-tree probably would be the most optimal, good for joins with fks
CREATE TABLE IF NOT EXISTS order_items
(
    id               SERIAL PRIMARY KEY,
    order_id         INTEGER NOT NULL,
    menu_position_id INTEGER NOT NULL,
    CONSTRAINT fk_order FOREIGN KEY (order_id)
        REFERENCES orders (id)
        ON DELETE CASCADE,
    CONSTRAINT fk_menu_position FOREIGN KEY (menu_position_id)
        REFERENCES menu (id)
        ON DELETE RESTRICT
)