use axum::{
    routing::{get, get_service, post, IntoMakeService},
    Router, Server,
};
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use hyper::server::conn::AddrIncoming;
use rand::Rng;
use sqlx::PgPool;
use std::{io, net::TcpListener};
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::routes::auth;

pub type App = Server<AddrIncoming, IntoMakeService<Router>>;

pub fn run(listener: TcpListener, pool: PgPool) -> hyper::Result<App> {
    let store = MemoryStore::new();
    let mut rng = rand::thread_rng();
    let secret: [u8; 128] = rng.gen();
    let session_layer = SessionLayer::new(store, &secret).with_secure(false);

    let auth_routes = Router::new()
        .route("/signup", post(auth::sign_up))
        .route("/login", post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/current", get(auth::current_user));

    let cors_layer = CorsLayer::default()
        .allow_credentials(true)
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
        ])
        .allow_origin(["http://localhost:5173".parse().unwrap()])
        .allow_headers([axum::http::header::ACCEPT, axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/health_check", get(crate::routes::health_check))
        .nest_service(
            "/tmp",
            get_service(ServeDir::new("tmp")).handle_error(|error: io::Error| async move {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .nest("/api", Router::new().nest("/auth", auth_routes))
        .with_state(pool)
        .layer(session_layer)
        .layer(cors_layer);

    Ok(Server::from_tcp(listener)?.serve(app.into_make_service()))
}
