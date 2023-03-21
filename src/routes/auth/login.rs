use crate::{
    io::error::AppError,
    repos::UsersRepository,
    services::{Claims, Password, Token},
};

use axum::{extract::State, response::IntoResponse, Json};
use axum_sessions::extractors::WritableSession;
use chrono::{Duration, Utc};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(serde::Deserialize, ToSchema)]
pub struct LoginRequest {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(serde::Serialize, ToSchema)]
pub struct LoginResponse {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) username: String,
}

#[utoipa::path(
        post,
        path = "/auth/login",
        request_body = LoginRequest,
        responses(
            (status = 200, description = "Successfully Logged In", body = LoginResponse),
        ),
        tag = "Auth"
    )]
pub async fn login(
    State(conn): State<PgPool>,
    mut session: WritableSession,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user_repository = UsersRepository { connection: &conn };

    let user = user_repository.find_user_with_email(&body.email).await?;

    if user.is_none() {
        return Err(AppError::BadRequest("Invalid email or password".into()));
    }

    let user = user.unwrap();

    let is_password_match =
        Password::verify_password(body.password.as_str(), user.password.as_str())?;

    if !is_password_match {
        return Err(AppError::BadRequest("Invalid email or password".into()));
    }

    let _date = Utc::now() + Duration::days(7);

    let token = Token::sign(Claims {
        sub: user.id,
        exp: _date.timestamp() as usize,
    })?;

    // Save jwt in cookie session
    session.insert("jwt", token)?;

    Ok((
        axum::http::StatusCode::OK,
        Json(LoginResponse {
            id: user.id,
            email: user.email,
            username: user.username,
        }),
    ))
}
