# Axum URL Shortener — Tutorial-Style

This project is a beginner-friendly URL shortener built with Rust, Axum, and MySQL. It focuses on the essentials: create a short link, redirect to the original URL, and fetch basic stats.

## What you will build

- POST /shorten to create a short link
- GET /{short_code} to redirect
- GET /stats/{short_code} to view clicks and created time
- GET /health to verify the database connection

## Prerequisites

- Rust (latest stable)
- MySQL Server

## Step 1: Create the database

```sql
CREATE DATABASE url_shortener_db;
```

## Step 2: Configure environment variables

Create a .env file in the project root:

```env
DATABASE_URL=mysql://username:password@localhost:3306/url_shortener_db
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
RUST_LOG=debug,tower_http=debug,axum=debug
```

## Step 3: Build and run

```bash
cargo build
cargo run
```

Server starts at http://127.0.0.1:3000

## Step 4: Try the API

### Health check

```bash
GET http://127.0.0.1:3000/health
```

### Create a short URL

```bash
POST http://127.0.0.1:3000/shorten
Content-Type: application/json

{
  "url": "https://www.example.com/very/long/url/that/needs/shortening"
}
```

Response example:

```json
{
  "short_code": "abc1234",
  "short_url": "http://127.0.0.1:3000/abc1234",
  "original_url": "https://www.example.com/very/long/url/that/needs/shortening"
}
```

### Redirect

```bash
GET http://127.0.0.1:3000/{short_code}
```

### Stats

```bash
GET http://127.0.0.1:3000/stats/{short_code}
```

Response example:

```json
{
  "short_code": "abc1234",
  "original_url": "https://www.example.com/very/long/url/that/needs/shortening",
  "clicks": 3,
  "created_at": "2026-02-10T18:23:45"
}
```

## How it works (quick tour)

1. main.rs starts the server and prepares shared state.
2. database.rs connects to MySQL and ensures the urls table exists.
3. routes.rs defines the handlers and SQL queries.

## Database schema

The app creates this table automatically if it does not exist:

```sql
CREATE TABLE urls (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  short_code VARCHAR(10) UNIQUE NOT NULL,
  original_url TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  clicks INT DEFAULT 0
);
```
