use rocket::response::Redirect;
use serde::{Serialize, Deserialize};
use oauth2::PkceCodeVerifier;
use json_dotpath::DotPaths;
use crate::github::*;
use crate::types::Session;
use crate::db::Connection;
use crate::models::user::User;

const PKCE_VERIFIER_PATH: &'static str = "PKCE_VERIFIER";
pub const GITHUB_ACCESS_TOKEN_PATH: &'static str = "GITHUB_ACCESS_TOKEN";

#[get("/login")]
pub fn login(session: Session) -> Redirect {
    let (authorize_url, _, _, pkce_verifier) = get_authorize_url();

    let _ = session.tap(|v| {
        v.dot_set(PKCE_VERIFIER_PATH, pkce_verifier.secret().clone())
    });

    Redirect::to(authorize_url.into_string())
}

#[get("/logout")]
pub fn logout(session: Session) -> Redirect {
    session.clear();
    Redirect::to("/")
}

#[derive(Debug, Serialize, Deserialize)]
struct GetAuthenticatedUserResponse {
    login: String,
    id: u64
}

#[get("/auth?<code>")]
pub fn authorize(code: String, session: Session, connection: Connection) -> Redirect {
    if let Some((token, user)) = session.tap(|data| {
        data.dot_get(PKCE_VERIFIER_PATH).ok().flatten().map(|verifier: serde_json::Value| { PkceCodeVerifier::new(verifier.as_str().unwrap().to_string()) })
    }).map(|pkce_verifier| { exchange_code_to_access_token(code, pkce_verifier) })
    .flatten()
    .map(|token| {
        create_api_client(&token.secret())
            .get("https://api.github.com/user")
            .send().map(|response| {
                response.json::<GetAuthenticatedUserResponse>().ok().map(|authenticated_user_response| { (token, authenticated_user_response) })
            }).ok().flatten()
    })
    .flatten() {
        let _ = session.tap(|data| {
            data.dot_set(GITHUB_ACCESS_TOKEN_PATH, token)
        });
        // アカウントが存在するか確認し、存在しなければ作成する
        let _ = User::find_by_github_id(user.id as i32, &connection);
    }
    
    Redirect::to(format!("/"))
}