use utoipa::{openapi::Server, Modify, OpenApi};
#[derive(OpenApi)]
#[openapi(
    modifiers(&ServerAddon),
    paths(
        crate::routes::health_check,
        crate::routes::auth::login,
        crate::routes::auth::sign_up,
        crate::routes::auth::current_user,
        crate::routes::auth::logout,
    ),
    components(schemas(
        crate::routes::auth::LoginRequest,
        crate::routes::auth::LoginResponse,

        crate::routes::auth::SignUpRequest,
        crate::routes::auth::SignUpResponse,

        crate::routes::auth::CurrentUser,
        crate::routes::auth::CurrentUserResponse,
    )),
    tags(
        (name = "Health Check", description = "Application Health Check"),
        (name = "Auth", description = "Auth Endpoints"),
    ),
)]
pub struct ApiDoc;

struct ServerAddon;

impl Modify for ServerAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.servers = Some(vec![Server::new("/api")])
    }
}
