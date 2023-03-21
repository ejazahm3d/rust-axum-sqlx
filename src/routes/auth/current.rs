use axum::{response::IntoResponse, Json};

use utoipa::ToSchema;
use uuid::Uuid;

use crate::extractors::AuthUser;

#[derive(serde::Serialize, ToSchema)]
pub struct CurrentUser {
    pub(crate) id: Uuid,
}

#[derive(serde::Serialize, ToSchema)]
pub struct CurrentUserResponse {
    pub(crate) user: Option<CurrentUser>,
}

#[utoipa::path(
        get,
        path = "/auth/current",
        responses(
            (status = 200, description = "Success", body = CurrentUserResponse),
        ),
        tag = "Auth"
    )]
pub async fn current_user(auth: Option<AuthUser>) -> impl IntoResponse {
    match auth {
        Some(_user) => Json(CurrentUserResponse {
            user: Some(CurrentUser { id: _user.id }),
        }),
        None => Json(CurrentUserResponse { user: None }),
    }
}
