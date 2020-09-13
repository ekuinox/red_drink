//! GitHubを使ったログインを実装している
//! /login -> GitHubにリダイレクト -> /authにリダイレクト -> CookieにJWTトークン詰めて / にリダイレクト
mod authorize;
mod login;

const PKCE_VERIFIER_PATH: &'static str = "PKCE_VERIFIER";
pub const GITHUB_ACCESS_TOKEN_PATH: &'static str = "GITHUB_ACCESS_TOKEN";

// re-exports
pub use self::authorize::*;
pub use self::login::*;
