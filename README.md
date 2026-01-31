# Axum API with MySQL

A REST API built with Rust, Axum web framework, and MySQL database using SeaORM.

## Prerequisites

- Rust (latest stable version)
- MySQL Server
- Cargo

## Setup Instructions

### 1. Install MySQL Server

Make sure MySQL is running on your system.

### 2. Create Database

```sql
CREATE DATABASE axum_db;
```

### 3. Configure Environment Variables

Copy the `.env.example` file to `.env` and update with your MySQL credentials:

```env
DATABASE_URL=mysql://username:password@localhost:3306/axum_db
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
RUST_LOG=debug,tower_http=debug,axum=debug
```

### 4. Install Dependencies

```bash
cargo build
```

### 5. Run the Server

```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`

## Available Endpoints

### Health Check

```bash
GET http://127.0.0.1:3000/health
```

Returns the API status and database connection status.

### Root

```bash
GET http://127.0.0.1:3000/
```

Returns a welcome message.

### Create User (Example)

```bash
POST http://127.0.0.1:3000/users
Content-Type: application/json

{
  "name": "John Doe",
  "email": "john@example.com"
}
```

## Tech Stack

- **Axum** - Web framework
- **SeaORM** - ORM for database operations
- **MySQL** - Database
- **Tokio** - Async runtime
- **Tower-HTTP** - Middleware (CORS, Tracing)
- **Serde** - Serialization/Deserialization

## Using SeaORM CLI

Generate entities from your database:

```bash
sea-orm-cli generate entity -o src/entities
```

Create migrations:

```bash
sea-orm-cli migrate init
sea-orm-cli migrate generate create_users_table
```

Run migrations:

```bash
sea-orm-cli migrate up
```

## Project Structure

```
src/
  ├── main.rs          # Application entry point
  ├── database.rs      # Database connection setup
  ├── routes.rs        # API routes and handlers
  └── entities/        # Generated SeaORM entities (after migration)
```

## Next Steps

1. Create database migrations for your tables
2. Generate entities using SeaORM CLI
3. Implement CRUD operations
4. Add authentication/authorization
5. Add input validation
6. Implement error handling middleware
