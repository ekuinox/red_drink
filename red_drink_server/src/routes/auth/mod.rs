//! GitHubを使ったログインを実装している
//! /login -> GitHubにリダイレクト -> /authにリダイレクト -> CookieにJWTトークン詰めて / にリダイレクト
use rocket::Route;
use crate::routes::Routes;


mod authorize;
mod login;

const PKCE_VERIFIER_PATH: &'static str = "PKCE_VERIFIER";

// re-exports
pub use self::authorize::*;
pub use self::login::*;

pub(crate) struct AuthRoutes;

impl Routes for AuthRoutes {
    fn routes() -> Vec<Route> {
        routes![authorize, login]
    }
}
