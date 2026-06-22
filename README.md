## A simple restaurant management application built with **Rust** and **Actix Web**, providing an API for managing orders and menu items.

---

## 🙏 Thank You for this opportunity

---

## ✨ Notes

- No AI was used during code implementation
- AI assistance was limited to SQL query optimization and this README formatting :)

---

## 🚀 How to Test

### 1. Build the application image

```bash
docker build --network=host -t paidy-restaurant-image:0.1 .
```
#### or
```bash
make build 
```
#### if 'make' is installed
### 2. Then

```bash
cargo test -- --test-threads=1
```
### Note: Tests currently run synchronously. Additional research is needed to enable asynchronous test execution.

---

## Design Decisions

### Order vs Table as Primary Entity

While the [spec](https://github.com/paidy/interview/blob/master/SimpleRestaurantApi.md) suggests `Table` as an entity, I chose `Order` as the main component because tables can have multiple simultaneous orders (e.g., drinks then food). Each order tracks its own items and status independently, with `table_number` as a field rather than a separate entity.

### Cooking Time

The application doesn't calculate cooking times — this belongs to the kitchen/cook's API, which has domain knowledge of preparation times and manages its own workflow.

## 📊 Domain Entities

# Restaurant Order Management System

### Roles
| Column      | Type    | Constraints        | Description                    |
|-------------|---------|--------------------|--------------------------------|
| id          | SERIAL  | PRIMARY KEY        | Unique identifier for the role |
| description | TEXT    |                    | Description of the role        |

### Users
| Column     | Type      | Constraints                | Description                              |
|------------|-----------|----------------------------|------------------------------------------|
| id         | SERIAL    | PRIMARY KEY                | Unique identifier for the user           |
| username   | TEXT      | UNIQUE, NOT NULL           | User's login name                        |
| password   | TEXT      | UNIQUE, NOT NULL           | Encrypted password (using pgcrypto)      |
| created_at | TIMESTAMP | DEFAULT CURRENT_TIMESTAMP  | Account creation timestamp               |
| role_id    | INTEGER   | NOT NULL, FOREIGN KEY      | References roles(id) - ON DELETE RESTRICT |

### Menu
| Column        | Type    | Constraints       | Description                               |
|---------------|---------|-------------------|-------------------------------------------|
| id            | SERIAL  | PRIMARY KEY       | Unique identifier for the menu item       |
| dish_title    | TEXT    | UNIQUE            | Name of the dish                          |
| cost          | BIGINT  | NOT NULL          | Price of the dish                         |
| time_to_cook  | INT     | NOT NULL          | Preparation time in minutes               |
| is_available  | BOOLEAN | DEFAULT TRUE      | Availability status of the dish           |

### Orders
| Column             | Type         | Constraints               | Description                                    |
|--------------------|--------------|---------------------------|------------------------------------------------|
| id                 | SERIAL       | PRIMARY KEY               | Unique identifier for the order                |
| table_number       | INTEGER      | NOT NULL                  | Table number for the order                     |
| waiter_assigned_id | INTEGER      | NOT NULL, FOREIGN KEY     | References users(id) - ON DELETE RESTRICT      |
| status             | order_status | NOT NULL                  | Current status of the order                    |
| created_at         | TIMESTAMP    | DEFAULT CURRENT_TIMESTAMP | Order creation timestamp                       |

**Order Status Enum Values:**
- `Created`
- `Cooking`
- `Ready`
- `Served`
- `Paid`
- `Cancelled`

### Order Items
| Column           | Type    | Constraints           | Description                                    |
|------------------|---------|-----------------------|------------------------------------------------|
| id               | SERIAL  | PRIMARY KEY           | Unique identifier for the order item           |
| order_id         | INTEGER | NOT NULL, FOREIGN KEY | References orders(id) - ON DELETE CASCADE      |
| menu_position_id | INTEGER | NOT NULL, FOREIGN KEY | References menu(id) - ON DELETE RESTRICT       |

## Entity Relationships

- **Roles** (1) ─── (N) **Users** - Each user has one role, a role can be assigned to multiple users
- **Users** (1) ─── (N) **Orders** - Each order is assigned to one waiter, a waiter can have multiple orders
- **Orders** (1) ─── (N) **Order Items** - Each order can contain multiple items, each item belongs to one order
- **Menu** (1) ─── (N) **Order Items** - Each menu item can appear in multiple order items, each order item references one menu item

## Authorization Model

The system uses a simplified role-based access control:

| Role    | Permissions                                                                 |
|---------|-----------------------------------------------------------------------------|
| Admin   | Full access to all operations                                               |
| Waiter  | Create orders for tables, modify orders assigned to them                    |
| Cook    | Modify menu items                                                           |

## Features

### 🔐 Authentication
Simple, locally implemented OAuth-like mechanism stored in the same database. Passwords are encrypted using PostgreSQL's `pgcrypto` extension.

### 📊 Observability & Metrics
Integrated **Prometheus** metrics for monitoring application health, request rates, latencies, and error rates — ready for production dashboards and alerting.

### ⚙️ Configuration Management
All secret and environment-specific values are managed through environment variables, with support for key-value stores. Configuration files reference these values securely, keeping secrets out of the codebase.

### Domain errors
All business logic is encapsulated in domain-specific errors,

---

## Testing

This project intentionally avoids traditional unit testing. The philosophy behind this approach:

- **Pure Functions Only:** Unit tests are reserved exclusively for pure functions — isolated logic without side effects, I/O, or database calls. This project has very little room for pure unit tests.
- **Everything Else Is Integration:** API endpoints, database queries, and service interactions are tested as integrated components
- **When abstraction is needed:** Database connections are abstracted behind traits, allowing lightweight stubs when unit-level isolation is needed for pure logic verification

### End-to-End Testing with TestContainers
All API and database tests use **TestContainers** to spin up real PostgreSQL instances in Docker, ensuring tests run against an actual database — no mocks, no surprises in production.

### Service Mocking with WireMock
When external services cannot be containerized (e.g., third-party APIs without Docker images), **WireMock** is used to simulate responses reliably.

---

## Infrastructure & Deployment

### Docker Compose
A `docker-compose.yml` file is provided for easy local development and testing.
- The application service
- PostgreSQL database

### Docker Build
Multi-stage Docker builds ensure small, secure production images.