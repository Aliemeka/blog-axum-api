# Blog API

A simple blog REST API built with Rust to learn Axum, SQLx, and shared state management.

## Stack

- **Axum** — web framework
- **SQLx** — async database queries (SQLite)
- **Tokio** — async runtime
- **Serde** — serialization/deserialization

## Features

- Create and list authors
- Create and list posts
- Filter posts by author

## Running
```bash
cp .env.example .env
cargo run
```

Server starts at `http://localhost:8080`.
