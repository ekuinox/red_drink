//! GitHubを使ったログインを実装している
//! /login -> GitHubにリダイレクト -> /authにリダイレクト -> CookieにJWTトークン詰めて / にリダイレクト
mod authorize;
mod login;

const PKCE_VERIFIER_PATH: &'static str = "PKCE_VERIFIER";

// re-exports
pub use self::authorize::*;
pub use self::login::*;
