use axum::{response::IntoResponse, Json};

use uuid::Uuid;

use crate::extractors::AuthUser;

#[derive(serde::Serialize)]
pub struct CurrentUser {
    pub(crate) id: Uuid,
}

#[derive(serde::Serialize)]
pub struct CurrentUserResponse {
    pub(crate) user: Option<CurrentUser>,
}

pub async fn current_user(auth: Option<AuthUser>) -> impl IntoResponse {
    match auth {
        Some(_user) => Json(CurrentUserResponse {
            user: Some(CurrentUser { id: _user.id }),
        }),
        None => Json(CurrentUserResponse { user: None }),
    }
}
