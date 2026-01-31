# Axum URL Shortener

A URL shortening service built with Rust, Axum web framework, and MySQL database using SeaORM. Convert long URLs into short, easy-to-share links.

## Prerequisites

- Rust (latest stable version)
- MySQL Server
- Cargo

## Setup Instructions

### 1. Install MySQL Server

Make sure MySQL is running on your system.

### 2. Create Database

```sql
CREATE DATABASE url_shortener_db;
```

### 3. Configure Environment Variables

Copy the `.env.example` file to `.env` and update with your MySQL credentials:

```env
DATABASE_URL=mysql://username:password@localhost:3306/url_shortener_db
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

### Shorten URL

```bash
POST http://127.0.0.1:3000/shorten
Content-Type: application/json

{
  "url": "https://www.example.com/very/long/url/that/needs/shortening"
}
```

Response:

```json
{
  "short_code": "abc123",
  "short_url": "http://127.0.0.1:3000/abc123",
  "original_url": "https://www.example.com/very/long/url/that/needs/shortening"
}
```

### Redirect to Original URL

```bash
GET http://127.0.0.1:3000/{short_code}
```

Redirects to the original URL associated with the short code.

### Get URL Statistics (Optional)

```bash
GET http://127.0.0.1:3000/stats/{short_code}
```

Returns statistics for a shortened URL (clicks, creation date, etc.).

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

## Features

- ✅ Shorten long URLs into compact codes
- ✅ Redirect short URLs to original destinations
- ✅ RESTful API design
- ✅ MySQL database with SeaORM
- ✅ Async/await with Tokio
- 🚧 URL statistics tracking (planned)
- 🚧 Custom short codes (planned)
- 🚧 Rate limiting (planned)
- 🚧 URL expiration (planned)

## Database Schema

The application uses a `urls` table with the following structure:

```sql
CREATE TABLE urls (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  short_code VARCHAR(10) UNIQUE NOT NULL,
  original_url TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  clicks INT DEFAULT 0
);
```
