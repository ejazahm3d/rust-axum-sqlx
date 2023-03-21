# Axum + SQLx + Postgres Project

This project is a basic example of a web application using the Axum framework for handling HTTP requests, SQLx for database interactions and Postgres as the database system. Simple auth example using JWT tokens

## Features

1. Axum, SQLx, Postgres
2. Integration Tests Setup
3. Simple Auth with JWT
4. OpenAPI integration along with rapidoc for displaying UI

## Requirements

- Rust
- Docker and Docker Compose
- SQLx CLI (installation instructions provided below)

## Setting up the Database

1. Make sure you have Docker and Docker Compose installed on your system.
2. Navigate to the project directory and run the following command to start the Postgres database:

```bash
docker-compose up -d
```

This will download the Postgres image and start a container with a new database. The `-d` flag runs the container in the background.

## Installing SQLx CLI

1. Make sure you have Rust installed on your system (version 1.56 or later).
2. Run the following command to install the SQLx CLI:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

This command installs the SQLx CLI with support for the Postgres database system.

## Building and Running the Project

1. Navigate to the project directory and run the following commands to copy .env file and run the database migrations:

```bash
cp .env.example env
sqlx migrate run
```

2. Run the following command to start the web server in development mode:

```bash
cargo run
```

This will start the web server and make it available at `http://localhost:5000`.

By default, the web server will run on `http://localhost:5000`. You can test the server by sending a GET request to `http://localhost:5000/api/health-check` using your preferred HTTP client.
You can go to `http://localhost:5000/rapidoc/docs` for OpenAPI docs.
