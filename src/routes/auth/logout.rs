use axum::response::IntoResponse;
use axum_sessions::extractors::WritableSession;

#[utoipa::path(
        post,
        path = "/auth/logout",
        responses(
            (status = 200, description = "Success"),
        ),
        tag = "Auth"
    )]
pub async fn logout(mut session: WritableSession) -> impl IntoResponse {
    session.destroy();
    axum::http::StatusCode::OK
}
