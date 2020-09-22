use rocket::Route;

pub mod api;
pub mod auth;

pub(crate) trait Routes {
    fn routes() -> Vec<Route>;
}

// re-exports
pub(crate) use auth::AuthRoutes;
pub(crate) use api::ApiRoutes;
