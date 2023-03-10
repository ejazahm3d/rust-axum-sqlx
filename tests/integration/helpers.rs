use rand::distributions::{Alphanumeric, DistString};
use reqwest::Client;
use rust_axum::{
    configuration::{get_configuration, DatabaseSettings},
    services::Password,
    startup::run,
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub client: Client,
    pub test_user: TestUser,
}

impl TestApp {
    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.client
            .post(&format!("{}/auth/login", self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.client
            .post(&format!("{}/auth/logout", self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_current_user(&self) -> reqwest::Response {
        self.client
            .get(&format!("{}/auth/current", self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}/api", port);

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;

        c
    };

    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    // Create and migrate the database

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let test_user = TestUser::generate();

    test_user.store(&connection_pool).await;

    let test_app = TestApp {
        address,
        db_pool: connection_pool,
        client,
        test_user,
    };

    return test_app;
}

#[derive(Debug, Clone)]
pub struct TestUser {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            email: format!("{}@example.com", Uuid::new_v4().to_string()),
            password: Uuid::new_v4().to_string(),
            firstname: Uuid::new_v4().to_string(),
            lastname: Uuid::new_v4().to_string(),
            username: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
        }
    }

    pub async fn login(&self, app: &TestApp) -> reqwest::Response {
        app.post_login(&serde_json::json!({
            "email": &self.email,
            "password": &self.password
        }))
        .await
    }

    pub async fn store(&self, pool: &PgPool) {
        let hashed_password = Password::hash_password(&self.password).unwrap();
        sqlx::query!(
            r#"
    INSERT INTO users(email, password, username)
    VALUES ($1, $2, $3)
    RETURNING *
    "#,
            self.email,
            hashed_password,
            self.username
        )
        .fetch_one(pool)
        .await
        .unwrap();
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
